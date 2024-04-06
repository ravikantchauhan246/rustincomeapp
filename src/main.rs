
use slint::SharedString;

slint::include_modules!();

const NECESSITIES: f64 = 0.40;
const DISCRETIONARY: f64= 0.30;
const DEBT: f64=0.20;
const CHARITY: f64=0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui:AppWindow = AppWindow::new()?;
    let ui_handle: slint::Weak<AppWindow>=ui.as_weak();
    ui.on_divide_income(move | string:SharedString|{

        let ui:AppWindow=ui_handle.unwrap();
        let num: f64=string.trim().parse().unwrap();
        let nec:f64=num*NECESSITIES;
        let dis:f64=num*DISCRETIONARY;
        let debt:f64=num*DEBT;
        let charity:f64=num*CHARITY;

        let result:String=format!("Necessities:{:.2}\nDiscretionary:{:.2}\nDebt:{:.2}\nCharity:{:.2}",nec,dis,debt,charity);
        ui.set_results(result.into())


    });
    

    
    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.run()
}
