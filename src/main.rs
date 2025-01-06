use glib::{ExitCode, g_info};
use gtk4::prelude::ApplicationExt;
use gtk4::{gio, glib, prelude::*};
use log::info;

const APP_ID: &str = "space.xkr47.gtk-log-redirect-bug";

fn main() -> ExitCode {
    env_logger::init();
    info!("rust Hello");

    glib::log_set_default_handler(glib::rust_log_handler);

    g_info!("main", "glib Hello");

    let app = gtk4::Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        MainWindow::new(app).show()
    });
    app.run()
}

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindowImp>)
        @extends gtk4::Widget, gtk4::Window, gtk4::ApplicationWindow,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget,
        gtk4::Native, gtk4::Root, gtk4::ShortcutManager, gio::ActionMap, gio::ActionGroup;
}

impl MainWindow {
    pub fn new<A: IsA<gtk4::Application>>(app: &A) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}

mod imp {
    use glib::g_info;
    use gtk4::{
        glib,
        subclass::prelude::*,
    };
    use gtk4::TemplateChild;

    #[derive(Debug, Default, gtk4::CompositeTemplate)]
    #[template(file = "ui.glade")]
    pub struct MainWindowImp {

        /// here, we look for label2 while the ui.glade file has only a "label1" component

        #[template_child]
        pub label2: TemplateChild<gtk4::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindowImp {
        const NAME: &'static str = "MainWindowImp";
        type Type = super::MainWindow;
        type ParentType = gtk4::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainWindowImp {
        fn constructed(&self) {
            g_info!("MainWindowImp", "glib constructed"); // not reached
            self.parent_constructed();
        }
    }

    impl WidgetImpl for MainWindowImp {}
    impl WindowImpl for MainWindowImp {}
    impl ApplicationWindowImpl for MainWindowImp {}
}
