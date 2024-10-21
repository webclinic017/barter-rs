use crate::{
    exchange::{
        gateio::{perpetual::trade::GateioFuturesTrades, Gateio},
        ExchangeServer, StreamSelector,
    },
    instrument::InstrumentData,
    subscription::trade::PublicTrades,
    transformer::stateless::StatelessTransformer,
    ExchangeWsStream, NoInitialSnapshots,
};
use barter_instrument::exchange::ExchangeId;

/// [`GateioOptions`] WebSocket server base url.
///
/// See docs: <https://www.gate.io/docs/developers/futures/ws/en/>
pub const WEBSOCKET_BASE_URL_GATEIO_OPTIONS_USD: &str = "wss://op-ws.gateio.live/v4/ws";

/// [`Gateio`] options exchange.
pub type GateioOptions = Gateio<GateioServerOptions>;

/// [`Gateio`] options [`ExchangeServer`].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct GateioServerOptions;

impl ExchangeServer for GateioServerOptions {
    const ID: ExchangeId = ExchangeId::GateioOptions;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_GATEIO_OPTIONS_USD
    }
}

impl<Instrument> StreamSelector<Instrument, PublicTrades> for GateioOptions
where
    Instrument: InstrumentData,
{
    type SnapFetcher = NoInitialSnapshots;
    type Stream = ExchangeWsStream<
        StatelessTransformer<Self, Instrument::Key, PublicTrades, GateioFuturesTrades>,
    >;
}
