use ribir::prelude::*;

// use crate::config::db::Database;

pub async fn exec_gui() {
    // let config = ForgeConfig::load()?;
    // let _db = Database::init().await.unwrap();

    let counter = fn_widget! {
      let cnt = Stateful::new(0);
      @Row {
        @FilledButton {
          on_tap: move |_| *$cnt.write() += 1,
          @{ Label::new("Inc") }
        }
        @H1 { text: pipe!($cnt.to_string()) }
      }
    };
    App::run(counter);
}
