mod block;
mod text;
mod list;
mod gauge;
mod sparkline;
mod chart;
mod barchart;

pub use self::block::Block;
pub use self::text::Text;
pub use self::list::List;
pub use self::gauge::Gauge;
pub use self::sparkline::Sparkline;
pub use self::chart::{Chart, Axis, Dataset};
pub use self::barchart::BarChart;

use buffer::Buffer;
use layout::Rect;
use terminal::Terminal;

pub mod border {
    bitflags! {
        pub flags Flags: u32 {
            const NONE  = 0b00000001,
            const TOP   = 0b00000010,
            const RIGHT = 0b00000100,
            const BOTTOM = 0b0001000,
            const LEFT = 0b00010000,
            const ALL = TOP.bits | RIGHT.bits | BOTTOM.bits | LEFT.bits,
        }
    }
}

pub trait Widget {
    fn buffer(&self, area: &Rect, buf: &mut Buffer);
    fn render(&self, area: &Rect, t: &mut Terminal)
        where Self: Sized
    {
        t.render(self, area);
    }
}