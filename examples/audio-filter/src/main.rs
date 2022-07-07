#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rtt_target::rprintln;

use stm32f4xx_hal as hal;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true,dispatchers = [EXTI0, EXTI1, EXTI2])]
mod app {
    use core::fmt::Write;

    use super::hal;

    use hal::gpio::gpioa::*;
    use hal::gpio::gpiob::*;
    use hal::gpio::gpioc::*;
    use hal::gpio::Edge;
    use hal::gpio::NoPin;
    use hal::i2s::stm32_i2s_v12x::driver::*;
    use hal::i2s::I2s;
    use hal::pac::Interrupt;
    use hal::pac::{EXTI, SPI2, SPI3};
    use hal::prelude::*;

    use heapless::spsc::*;

    use rtt_target::{rprintln, rtt_init, set_print_channel};

    type I2s2Driver = I2sDriver<I2s<SPI2, (PB12, PB13, PC6, PB15)>, Master, Receive, Philips>;
    type I2s3Driver = I2sDriver<I2s<SPI3, (PA4, PC10, NoPin, PC12)>, Slave, Transmit, Philips>;

    // Part of the frame we currently transmit or receive
    #[derive(Copy, Clone)]
    pub enum FrameState {
        LeftMsb,
        LeftLsb,
        RightMsb,
        RightLsb,
    }

    use FrameState::{LeftLsb, LeftMsb, RightLsb, RightMsb};

    impl Default for FrameState {
        fn default() -> Self {
            Self::LeftMsb
        }
    }
    #[shared]
    struct Shared {
        #[lock_free]
        i2s2_driver: I2s2Driver,
        #[lock_free]
        i2s3_driver: I2s3Driver,
        #[lock_free]
        exti: EXTI,
    }

    #[local]
    struct Local {
        logs_chan: rtt_target::UpChannel,
        adc_p: Producer<'static, (i32, i32), 2>,
        process_c: Consumer<'static, (i32, i32), 2>,
        process_p: Producer<'static, (i32, i32), 2>,
        dac_c: Consumer<'static, (i32, i32), 2>,
    }

    #[init(local = [queue_1: Queue<(i32,i32), 2> = Queue::new(),queue_2: Queue<(i32,i32), 2> = Queue::new()])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let queue_1 = cx.local.queue_1;
        let queue_2 = cx.local.queue_2;
        let channels = rtt_init! {
            up: {
                0: {
                    size: 128
                    name: "Logs"
                }
                1: {
                    size: 128
                    name: "Panics"
                }
            }
        };
        let logs_chan = channels.up.0;
        let panics_chan = channels.up.1;
        set_print_channel(panics_chan);
        let (adc_p, process_c) = queue_1.split();
        let (process_p, dac_c) = queue_2.split();
        let device = cx.device;
        let mut syscfg = device.SYSCFG.constrain();
        let mut exti = device.EXTI;
        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();
        let rcc = device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8u32.MHz())
            .sysclk(96.MHz())
            .hclk(96.MHz())
            .pclk1(50.MHz())
            .pclk2(100.MHz())
            .i2s_clk(61440.kHz())
            .freeze();

        // I2S pins: (WS, CK, MCLK, SD) for I2S2
        let i2s2_pins = (
            gpiob.pb12, //WS
            gpiob.pb13, //CK
            gpioc.pc6,  //MCK
            gpiob.pb15, //SD
        );
        let i2s2 = I2s::new(device.SPI2, i2s2_pins, &clocks);
        let i2s2_config = I2sDriverConfig::new_master()
            .receive()
            .standard(Philips)
            .data_format(DataFormat::Data24Channel32)
            .master_clock(true)
            .request_frequency(48_000);
        let mut i2s2_driver = I2sDriver::new(i2s2, i2s2_config);
        rprintln!("actual sample rate is {}", i2s2_driver.sample_rate());
        i2s2_driver.set_rx_interrupt(true);
        i2s2_driver.set_error_interrupt(true);

        // I2S3 pins: (WS, CK, NoPin, SD) for I2S3
        let i2s3_pins = (gpioa.pa4, gpioc.pc10, NoPin, gpioc.pc12);
        let i2s3 = I2s::new(device.SPI3, i2s3_pins, &clocks);
        let i2s3_config = i2s2_config.to_slave().transmit();
        let mut i2s3_driver = I2sDriver::new(i2s3, i2s3_config);
        i2s3_driver.set_tx_interrupt(true);
        i2s3_driver.set_error_interrupt(true);

        // set up an interrupt on WS pin
        let ws_pin = i2s3_driver.i2s_peripheral_mut().ws_pin_mut();
        ws_pin.make_interrupt_source(&mut syscfg);
        ws_pin.trigger_on_edge(&mut exti, Edge::Rising);
        // we will enable i2s3 in interrupt
        ws_pin.enable_interrupt(&mut exti);

        i2s2_driver.enable();

        (
            Shared {
                i2s2_driver,
                i2s3_driver,
                exti,
            },
            Local {
                logs_chan,
                adc_p,
                process_c,
                process_p,
                dac_c,
            },
            init::Monotonics(),
        )
    }

    #[idle(shared = [], local = [])]
    fn idle(_cx: idle::Context) -> ! {
        #[allow(clippy::empty_loop)]
        loop {}
    }

    // Printing message directly in a i2s interrupt can cause timing issues.
    #[task(capacity = 10, local = [logs_chan])]
    fn log(cx: log::Context, message: &'static str) {
        writeln!(cx.local.logs_chan, "{}", message).unwrap();
    }

    // processing audio
    #[task(binds = SPI1, local = [count: u32 = 0,process_c,process_p])]
    fn process(cx: process::Context) {
        let count = cx.local.count;
        let process_c = cx.local.process_c;
        let process_p = cx.local.process_p;
        while let Some(mut smpl) = process_c.dequeue() {
            let period = 24000;
            if *count > period / 2 {
                smpl.0 >>= 1;
            }
            if *count > period / 4 && *count <= period * 3 / 4 {
                smpl.1 >>= 1;
            }
            *count += 1;
            if *count >= period {
                *count = 0;
            }
            process_p.enqueue(smpl).ok();
        }
    }

    #[task(
        priority = 4,
        binds = SPI2,
        local = [frame_state: FrameState = LeftMsb, frame: (u32,u32) = (0,0),adc_p],
        shared = [i2s2_driver]
    )]
    fn i2s2(cx: i2s2::Context) {
        let frame_state = cx.local.frame_state;
        let frame = cx.local.frame;
        let adc_p = cx.local.adc_p;
        let i2s2_driver = cx.shared.i2s2_driver;
        let status = i2s2_driver.status();
        // It's better to read first to avoid triggering ovr flag
        if status.rxne() {
            let data = i2s2_driver.read_data_register();
            match (*frame_state, status.chside()) {
                (LeftMsb, Channel::Left) => {
                    frame.0 = (data as u32) << 16;
                    *frame_state = LeftLsb;
                }
                (LeftLsb, Channel::Left) => {
                    frame.0 |= data as u32;
                    *frame_state = RightMsb;
                }
                (RightMsb, Channel::Right) => {
                    frame.1 = (data as u32) << 16;
                    *frame_state = RightLsb;
                }
                (RightLsb, Channel::Right) => {
                    frame.1 |= data as u32;
                    // defer sample processing to another task
                    let (l, r) = *frame;
                    adc_p.enqueue((l as i32, r as i32)).ok();
                    rtic::pend(Interrupt::SPI1);
                    *frame_state = LeftMsb;
                }
                // in case of ovr this resynchronize at start of new frame
                _ => *frame_state = LeftMsb,
            }
        }
        if status.ovr() {
            log::spawn("i2s2 Overrun").ok();
            // sequence to delete ovr flag
            i2s2_driver.read_data_register();
            i2s2_driver.status();
        }
    }

    #[task(
        priority = 4,
        binds = SPI3,
        local = [frame_state: FrameState = LeftMsb,frame: (u32,u32) = (0,0),dac_c],
        shared = [i2s3_driver,exti]
    )]
    fn i2s3(cx: i2s3::Context) {
        let frame_state = cx.local.frame_state;
        let frame = cx.local.frame;
        let dac_c = cx.local.dac_c;
        let i2s3_driver = cx.shared.i2s3_driver;
        let exti = cx.shared.exti;
        let status = i2s3_driver.status();
        // it's better to write data first to avoid to trigger udr flag
        if status.txe() {
            let data;
            match (*frame_state, status.chside()) {
                (LeftMsb, Channel::Left) => {
                    let (l, r) = dac_c.dequeue().unwrap_or_default();
                    *frame = (l as u32, r as u32);
                    data = (frame.0 >> 16) as u16;
                    *frame_state = LeftLsb;
                }
                (LeftLsb, Channel::Left) => {
                    data = (frame.0 & 0xFFFF) as u16;
                    *frame_state = RightMsb;
                }
                (RightMsb, Channel::Right) => {
                    data = (frame.1 >> 16) as u16;
                    *frame_state = RightLsb;
                }
                (RightLsb, Channel::Right) => {
                    data = (frame.1 & 0xFFFF) as u16;
                    *frame_state = LeftMsb;
                }
                // in case of udr this resynchronize tracked and actual channel
                _ => {
                    *frame_state = LeftMsb;
                    data = 0; //garbage data to avoid additional underrrun
                }
            }
            i2s3_driver.write_data_register(data);
        }
        if status.fre() {
            log::spawn("i2s3 Frame error").ok();
            i2s3_driver.disable();
            i2s3_driver
                .i2s_peripheral_mut()
                .ws_pin_mut()
                .enable_interrupt(exti);
        }
        if status.udr() {
            log::spawn("i2s3 udr").ok();
            i2s3_driver.status();
            i2s3_driver.write_data_register(0);
        }
    }

    // Look i2s3 WS line for (re) synchronisation
    #[task(priority = 4, binds = EXTI4, shared = [i2s3_driver,exti])]
    fn exti4(cx: exti4::Context) {
        let i2s3_driver = cx.shared.i2s3_driver;
        let exti = cx.shared.exti;
        let ws_pin = i2s3_driver.i2s_peripheral_mut().ws_pin_mut();
        ws_pin.clear_interrupt_pending_bit();
        // yes, in this case we already know that pin is high, but some other exti can be triggered
        // by several pins
        if ws_pin.is_high() {
            ws_pin.disable_interrupt(exti);
            i2s3_driver.write_data_register(0);
            i2s3_driver.enable();
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}
