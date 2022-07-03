#![no_main]
#![no_std]

use rtic::app;

#[app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    // Lustre Model
    use blinky_implem::Blinky;
    use blinky_implem::Lustre::System;

    // RTIC + HAL
    use panic_semihosting as _;
    use stm32f4xx_hal::gpio::{Edge, Input, Output, PushPull, PA0, PD12, PD13, PD14, PD15};
    use stm32f4xx_hal::prelude::*;
    use systick_monotonic::{fugit::Duration, Systick};

    #[shared]
    struct Shared {
        button_pressed: bool,
    }

    #[local]
    struct Local {
        leds: (
            PD12<Output<PushPull>>,
            PD13<Output<PushPull>>,
            PD14<Output<PushPull>>,
            PD15<Output<PushPull>>,
        ),
        button: PA0<Input>,
        // Lustre Model Here
        sys: Blinky,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut dp = cx.device;

        // Setup clocks
        let rcc = dp.RCC.constrain();

        let mono = Systick::new(cx.core.SYST, 36_000_000);

        let _clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(36.MHz()).pclk1(36.MHz());

        let mut syscfg = dp.SYSCFG.constrain();

        // Setup LED
        let gpiod = dp.GPIOD.split();
        let led0 = gpiod.pd12.into_push_pull_output();
        let led1 = gpiod.pd13.into_push_pull_output();
        let led2 = gpiod.pd14.into_push_pull_output();
        let led3 = gpiod.pd15.into_push_pull_output();

        // Setup Push Button
        let gpioa = dp.GPIOA.split();
        let mut board_btn = gpioa.pa0.into_pull_down_input();
        board_btn.make_interrupt_source(&mut syscfg);
        board_btn.enable_interrupt(&mut dp.EXTI);
        board_btn.trigger_on_edge(&mut dp.EXTI, Edge::Falling);

        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(10)).unwrap();

        (
            Shared {
                button_pressed: false,
            },
            Local {
                leds: (led0, led1, led2, led3),
                button: board_btn,
                sys: Blinky::init((false,)).unwrap(),
            },
            init::Monotonics(mono),
        )
    }

    /// EXTI0 comes from push button and submits a toggle request
    #[task(binds=EXTI0, shared=[button_pressed], local=[button,])]
    fn button(cx: button::Context) {
        // set toggle signal
        let button::Context { mut shared, local } = cx;
        local.button.clear_interrupt_pending_bit();
        shared.button_pressed.lock(|btn| {
            *btn = true;
        });
    }

    /// Task Updates the Lustre Blinky Model and Sets the LEDs
    #[task(local = [leds, sys], shared = [button_pressed])]
    fn blink(cx: blink::Context) {
        // attend to toggle signal
        let blink::Context { mut shared, local } = cx;
        shared.button_pressed.lock(|btn| {
            local.sys.next((*btn,)).unwrap();
            *btn = false;
        });

        // map the system output to leds
        let (index, _) = local.sys.output();
        match index {
            0 => {
                local.leds.0.set_high();
                local.leds.1.set_low();
                local.leds.2.set_low();
                local.leds.3.set_low();
            }
            1 => {
                local.leds.1.set_high();
                local.leds.0.set_low();
                local.leds.2.set_low();
                local.leds.3.set_low();
            }
            2 => {
                local.leds.2.set_high();
                local.leds.0.set_low();
                local.leds.1.set_low();
                local.leds.3.set_low();
            }
            3 => {
                local.leds.3.set_high();
                local.leds.0.set_low();
                local.leds.1.set_low();
                local.leds.2.set_low();
            }
            _ => (),
        }

        // setup the next interrupt
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(50)).unwrap();
    }
}
