extern crate embedded_graphics;
extern crate embedded_hal;
extern crate linux_embedded_hal as hal;
extern crate mio_httpc;
#[macro_use]
extern crate serde;
extern crate ssd1306;

use std::thread;
use std::time::Duration;

use embedded_graphics::fonts::Font12x16;
use embedded_graphics::prelude::*;
use hal::*;
use mio_httpc::CallBuilder;
use ssd1306::prelude::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Time {
    current_date_time: String,
    current_file_time: u64,
    day_of_the_week: String,
    is_day_light_savings_time: bool,
    ordinal_date: String,
    time_zone_name: String,
    utc_offset: String,
}

fn main() -> Result<(), mio_httpc::Error> {
    let i2c = I2cdev::new("/dev/i2c-1").expect("i2c1");

    let mut disp: GraphicsMode<_> = ssd1306::Builder::new()
        .with_i2c_addr(0x3c)
        .connect_i2c(i2c)
        .into();

    disp.init().unwrap();
    disp.flush().unwrap();
    disp.clear();

    loop {
        thread::sleep(Duration::from_millis(1000));

        CallBuilder::get()
            .timeout_ms(5000)
            .url("http://worldclockapi.com/api/json/pst/now")?
            .exec()
            .map(|(_, body)| {
                serde_json::from_slice(&body)
                    .map(|time: Time| {
                        let s: Vec<&str> = time.current_date_time.split("T").skip(1).collect();
                        let s: Vec<&str> = s[0].split("-").take(1).collect();

                        disp.draw(Font12x16::render_str(s[0]).into_iter());
                        disp.flush().unwrap();
                    })
                    .map_err(|x| {
                        eprintln!("can't parse JSON {:?}", x);
                    });
            })
            .map_err(|x| {
                eprintln!("request lib error {:?}", x);
            });
    }
}
