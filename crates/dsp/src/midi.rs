use defmt::warn;
use rtic_sync::channel;
use stm32h7xx_hal::{nb, pac, prelude::_embedded_hal_serial_Read, serial};

pub const MIDI_RX_CAPACITY: usize = 8;

pub type MidiRxSender = channel::Sender<'static, u8, MIDI_RX_CAPACITY>;
pub type MidiRxReceiver = channel::Receiver<'static, u8, MIDI_RX_CAPACITY>;

pub fn enqueue_midi_processing(
    midi_rx: &mut serial::Rx<pac::USART3>,
    midi_rx_send: &mut MidiRxSender,
) {
    match midi_rx.read() {
        Ok(byte) => {
            match midi_rx_send.try_send(byte) {
                Ok(()) => {}
                Err(channel::TrySendError::NoReceiver(_)) => {
                    // todo handle the case
                    warn!("MIDI RX send: NoReceiver");
                }
                Err(channel::TrySendError::Full(_)) => {
                    // todo handle the case
                    warn!("MIDI RX send: Full");
                }
            };
        }
        Err(err) => match err {
            nb::Error::Other(err) => match err {
                serial::Error::Framing => warn!("MIDI RX error: Framing"),
                serial::Error::Noise => warn!("MIDI RX error: Noise"),
                serial::Error::Overrun => warn!("MIDI RX error: Overrun"),
                serial::Error::Parity => warn!("MIDI RX error: Parity"),
                _ => warn!("MIDI RX error: Unknown"),
            },
            nb::Error::WouldBlock => warn!("MIDI RX error: Would block"),
        },
    }
}
