use yew::prelude::*;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, route::Route};
use std::sync::RwLockReadGuard;

use crate::chip_library::{ChipLibrary, get_instance};
use crate::util::timeout::{set_timeout, TimeoutHandle};



#[derive(PartialEq, Eq)]
pub enum Tabs {
    Library,
    Pack,
    Folder,
    GroupFolder(String),
}


impl std::fmt::Display for Tabs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tabs::Library => write!(f, "Library"),
            Tabs::Folder => write!(f, "Folder"),
            Tabs::Pack => write!(f, "Pack"),
            Tabs::GroupFolder(name) => write!(f, "{}'s Folder", name),
        }
    }
}

pub enum TopLevelMsg {
    ChangeTab(Tabs),
    SetMsg(String),
}

/// Root component
pub struct App
{
    active_tab: Tabs,
    link: ComponentLink<Self>,
    message_txt: String,
    message_clear_timeout_handle: Option<TimeoutHandle>,
}

impl App {

    /// change the active tab, returns true if the new tab is different
    fn change_tab(&mut self, tab: Tabs) -> bool {

        if self.active_tab == tab {
            return false;
        }
        self.active_tab = tab;
        true
    }

    /// change the current message, returns true for consistency with change_tab
    fn set_message(&mut self, message: String) -> bool {
        
        if message == "" {
            let handle = self.message_clear_timeout_handle.take();
            drop(handle);
        } else {
            self.set_message_clear_timeout();
        }

        self.message_txt = message;
        true
    }

    fn set_message_clear_timeout(&mut self) {


        //ensure that previous timeout is cancelled
        let old_timeout = self.message_clear_timeout_handle.take();
        drop(old_timeout);

        let callback = self.link.callback_once(|_: ()| TopLevelMsg::SetMsg("".to_owned()));
        
        self.message_clear_timeout_handle = Some(set_timeout(15000, move || callback.emit(())).unwrap());
        
    }

}

impl Component for App {
    type Message = TopLevelMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            active_tab: Tabs::Library,
            message_txt: "test msg".to_owned(),
            message_clear_timeout_handle: None,
            link,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        return match msg {
            TopLevelMsg::ChangeTab(tab) => self.change_tab(tab),
            TopLevelMsg::SetMsg(message) => self.set_message(message),
        }
    }

    fn view(&self) -> Html {
        

        html! {
            <div class="container-fluid" style="background-color: #00637b; padding: 5px; max-width: 720px">
                <div style="background-color: #ffbd18; font-family: Lucida Console; margin: 5px; color: #FFFFFF; font-weight: bold">
                    <span style="padding-left: 5px">{self.active_tab.to_string()}</span><span style="float: right; color: red">{self.message_txt.to_string()}</span>
                </div>
            </div>
        }

        //let library: RwLockReadGuard<ChipLibrary> = get_instance().get().unwrap().read().unwrap();
        
        /*
        html! {
            <>
            {
                library.library.iter().map(|chip| html!{
                    <div>
                        {&chip.1.name}{" "}{chip.1.kind}
                    </div>
                }).collect::<Html>()
            }
            </>
        }
        */



        /*
        html! {
            <>
                <Nav />
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute | {
                        match switch {
                            AppRoute::Home => html!{ <Home /> },
                            AppRoute::About => html!{ <About /> },
                            AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRoute::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </>
        }
        */
    }
}
