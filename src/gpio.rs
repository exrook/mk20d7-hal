use core::marker::PhantomData;
use mk20d7::sim::SCGC5;

pub trait GpioExt {
    type Parts;
    fn split(self, scgc5: &SCGC5) -> Self::Parts;
}

pub struct Inactive;
pub struct Input<MODE> {
    _mode: PhantomData<MODE>
}
pub struct Floating;
pub struct Output<MODE> {
    _mode: PhantomData<MODE>
}
pub struct OpenDrain;
pub struct PushPull;


macro_rules! gpio {
    ($PORTX:ident, $portx:ident, $PTX:ident, $gpiox:ident, [ $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+]) =>
    {
        pub mod $gpiox {
            use core::marker::PhantomData;

            use embedded_hal::digital::OutputPin;

            use mk20d7::{$PORTX, $PTX};
            use mk20d7::$portx::PCR;
            use mk20d7::sim::SCGC5;

            use super::{Input, Output, Inactive, Floating, PushPull, OpenDrain, GpioExt};

            pub struct Parts {
                //_gpclr: GPCLR,
                //_gpchr: GPCHR,
                //_isfr: ISFR,
                //_pddr: PDDR,
                $(
                pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for ($PTX, $PORTX) {
                type Parts = Parts;
                fn split(self, scgc5: &SCGC5) -> Self::Parts {
                    // Enable the GPIO module
                    scgc5.modify(|_,w|w.$portx().set_bit());
                    Parts {
                        //// These registers are dangerous to keep around because they can modify pins we
                        //// have already moved out
                        //_gpclr: GPCLR{},
                        //_gpchr: GPCHR{},
                        //// This is also a global register, see above
                        //_isfr: ISFR{},
                        //_pddr: PDDR{},
                        $(
                        $pxi: $PXi {_mode: PhantomData},
                        )+
                    }
                }
            }

            //// These come from PORTx
            ///// Global Pin Control Low Register
            //struct GPCLR {}
            ///// Global Pin Control High Register
            //struct GPCHR {}
            ///// Interrupt Status Flag Register
            //struct ISFR {}

            // I was not able to find documentation on these registers outside of the svd
            ///// Digital Filter Enable Register
            //struct DFER {}
            ///// Digital Filter Clock Register
            //struct DFCR {}
            ///// Digital Filter Width Register
            //struct DFWR {}

            // These are found in PTx
            ///// Port Data Output Register
            //struct PDOR {}
            ///// Port Set Output Register
            //struct PSOR {}
            ///// Port Clear Output Register
            //struct PCOR {}
            ///// Port Toggle Output Register
            //struct PTOR {}
            ///// Port Data Input Register
            //struct PDIR {}

            ///// Port Data Direction Register
            //struct PDDR {}

            // This pin owns its section of the PDOR, PSOR, PCOR, PTOR, and PDIR registers, as well as its
            // PCR register
            $(
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }
                impl<MODE> $PXi<MODE> {
                    pub(crate) fn pcr(&mut self) -> &PCR {
                        unsafe {&(*$PORTX::ptr()).pcr[$i]}
                    }
                    pub fn into_push_pull_output(self) -> $PXi<Output<PushPull>> {
                        // Set the pin to mode 1 (GPIO), and disable Open Drain mode
                        unsafe {(*$PORTX::ptr()).pcr[$i].modify(|_,w|w.mux()._001().ode().clear_bit().dse().set_bit().sre().set_bit())}
                        // Set the pin to output mode
                        unsafe {(*$PTX::ptr()).pddr.modify(|r,w|w.bits(r.bits()|1<<$i))};
                        $PXi { _mode: PhantomData }
                    }
                }
                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn is_high(&self) -> bool {
                        if unsafe {(*$PTX::ptr()).pdor.read().bits()} & (1<<$i) != 0 {
                            true
                        } else {
                            false
                        }
                    }
                    fn is_low(&self) -> bool {
                        if unsafe {(*$PTX::ptr()).pdor.read().bits()} & (1<<$i) != 0 {
                            false
                        } else {
                            true
                        }
                    }
                    fn set_low(&mut self) {
                        unsafe {(*$PTX::ptr()).pcor.write(|w|w.bits(1<<$i))};
                    }
                    fn set_high(&mut self) {
                        unsafe {(*$PTX::ptr()).psor.write(|w|w.bits(1<<$i))};
                    }
                }
            )+
        }
    }
}
gpio!(PORTA, porta, PTA, gpioa, [
      PA0: (pa0, 0, Inactive), //JTAG_TCLK/SWD_CLK/EZP_CLK
      PA1: (pa1, 1, Inactive), //JTAG_TDI/EZP_DI
      PA2: (pa2, 2, Inactive), //JTAG_TDO/TRACE_SWO/EZP_DO
      PA3: (pa3, 3, Inactive), //JTAG_TMS/SWD_DIO
      PA4: (pa4, 4, Inactive), //NMI_b/EZP_CS_b
      PA5: (pa5, 5, Inactive), //Disabled
      PA12: (pa12, 12, Inactive), //CMP2_IN0
      PA13: (pa13, 13, Inactive), //CMP2_IN1
      PA18: (pa18, 18, Inactive), //EXTAL0
      PA19: (pa19, 19, Inactive), //XTAL0
]);
gpio!(PORTB, portb, PTB, gpiob, [
      PB0: (pb0, 0, Inactive), //ADC0_SE8/ADC1_SE8/TSI0_CH0
      PB1: (pb1, 1, Inactive), //ADC0_SE9/ADC1_SE9/TSI0_CH6
      PB2: (pb2, 2, Inactive), //ADC0_SE12/TSI0_CH7
      PB3: (pb3, 3, Inactive), //ADC0_SE13/TSI0_CH8
      PB16: (pb16, 16, Inactive), //TSI0_CH9
      PB17: (pb17, 17, Inactive), //TSI0_CH10
      PB18: (pb18, 18, Inactive), //TSI0_CH11
      PB19: (pb19, 19, Inactive), //TSI0_CH12
]);
gpio!(PORTC, portc, PTC, gpioc, [
      PC0: (pc0, 0, Inactive), //ADC0_SE14/TSI0_CH13
      PC1: (pc1, 1, Inactive), //ADC0_SE15/TSI0_CH14
      PC2: (pc2, 2, Inactive), //ADC0_SE4b/CMP1_IN0/TSI0_CH15
      PC3: (pc3, 3, Inactive), //CMP1_IN1
      PC4: (pc4, 4, Inactive), //Disabled
      PC5: (pc5, 5, Inactive), //Disabled
      PC6: (pc6, 6, Inactive), //CMP0_IN0
      PC7: (pc7, 7, Inactive), //CMP0_IN1
      PC8: (pc8, 8, Inactive), //ADC1_SE4b/CMP0_IN2
      PC9: (pc9, 9, Inactive), //ADC1_SE5b/CMP0_IN3
      PC10: (pc10, 10, Inactive), //ADC1_SE6b
      PC11: (pc11, 11, Inactive), //ADC1_SE7b
]);
gpio!(PORTD, portd, PTD, gpiod, [
      PD0: (pd0, 0, Inactive), //Disabled
      PD1: (pd1, 1, Inactive), //ADC0_SE5b
      PD2: (pd2, 2, Inactive), //Disabled
      PD3: (pd3, 3, Inactive), //Disabled
      PD4: (pd4, 4, Inactive), //Disabled
      PD5: (pd5, 5, Inactive), //ADC0_SE6b
      PD6: (pd6, 6, Inactive), //ADC0_SE7b
      PD7: (pd7, 7, Inactive), //Disabled
]);
gpio!(PORTE, porte, PTE, gpioe, [
      PE0: (pe0, 0, Inactive), //ADC1_SE4a
      PE1: (pe1, 1, Inactive), //ADC1_SE5a
]);
