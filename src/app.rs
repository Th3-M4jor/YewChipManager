use yew::prelude::*;

use crate::util::timeout::{set_timeout, TimeoutHandle};
use crate::components::library::LibraryComponent as Library;



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

    fn gen_nav_tabs(&self) -> Html {

        return match self.active_tab {

            Tabs::Library => {
                html! {
                    <div class="btn-group" role="tabs" style="padding-left: 20px; transform: translate(0px,6px)">
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Folder))>{"Folder"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Pack))>{"Pack"}</button>
                        <button class="btn activeNavTab">{"Library"}</button>
                    </div>
                }
            }
            Tabs::Pack => {
                html! {
                    <div class="btn-group" role="tabs" style="padding-left: 20px; transform: translate(0px,6px)">
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Folder))>{"Folder"}</button>
                        <button class="btn activeNavTab">{"Pack"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Library))>{"Library"}</button>
                    </div>
                }
            }
            Tabs::Folder => {
                html! {
                    <div class="btn-group" role="tabs" style="padding-left: 20px; transform: translate(0px,6px)">
                        <button class="btn activeNavTab">{"Folder"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Pack))>{"Pack"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Library))>{"Library"}</button>
                    </div>
                }
            }
            _ => { 
                html! {
                    <div/>
                }
            }
        };
        
    }

}

impl Component for App {
    type Message = TopLevelMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            active_tab: Tabs::Library,
            message_txt: "".to_owned(),
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

        let set_msg_callback = self.link.callback(|msg: String| TopLevelMsg::SetMsg(msg));
        

        html! {
            <div class="container-fluid" style="background-color: #00637b; padding: 5px; max-width: 720px">
                <div style="background-color: #ffbd18; font-family: Lucida Console; margin: 5px; color: #FFFFFF; font-weight: bold">
                    <span style="padding-left: 5px">{&self.active_tab}</span><span style="float: right; color: red">{&self.message_txt}</span>
                </div>
                <div style="background-color: #4abdb5; padding: 10px">
                    {self.gen_nav_tabs()}
                    <Library active={self.active_tab == Tabs::Library} set_msg_callback={set_msg_callback}/>
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
