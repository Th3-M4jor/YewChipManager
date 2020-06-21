use yew::prelude::*;
use yew::services::{
    reader::{ReaderService, ReaderTask, FileData},
    timeout::{TimeoutService, TimeoutTask},
    interval::{IntervalService, IntervalTask},
};
use std::borrow::Cow;
use std::time::Duration;

use crate::util::{storage_available, alert};
use crate::components::{library::LibraryComponent as Library, pack::PackComponent as Pack, folder::FolderComponent as Folder, chip_desc::ChipDescComponent as ChipDescBox};
use crate::agents::{
    global_msg::{GlobalMsgBus, Request as GlobalReq},
    group_folder::{GroupFldrMsgBus, GroupFldrAgentOutMsg, GroupFldrAgentReq},
};
use crate::chip_library::ChipLibrary;

use wasm_bindgen::JsCast;


#[derive(PartialEq, Eq, Clone)]
pub enum Tabs {
    Library,
    Pack,
    Folder,
    GroupFolder(String),
}

impl Tabs {
    pub fn shorten_string(&self) -> Cow<str> {
        match self {
            Tabs::Library => Cow::Borrowed("Lib"),
            Tabs::Pack => Cow::Borrowed("Pck"),
            Tabs::Folder => Cow::Borrowed("Fldr"),
            Tabs::GroupFolder(grp_fldr) => {
                let mut text = String::new();
                text.push_str(&grp_fldr[..=4]);
                text.push_str("...");
                Cow::Owned(text)
            }
        }
    }

    pub fn to_display_text(&self) -> Cow<str> {
        match self {
            Tabs::Library => Cow::Borrowed("Library"),
            Tabs::Pack => Cow::Borrowed("Pack"),
            Tabs::Folder => Cow::Borrowed("Library"),
            Tabs::GroupFolder(grp_fldr) => {
                let mut text = String::from(grp_fldr);
                text.push_str("'s folder");
                Cow::Owned(text)
            }
        }
    }
}

impl PartialEq<str> for Tabs {
    fn eq(&self, other: &str) -> bool {
        match self {
            Tabs::Library => {"Library" == other}
            Tabs::Pack => {"Pack" == other}
            Tabs::Folder => {"Folder" == other}
            Tabs::GroupFolder(grp_fldr) => {
                grp_fldr == other
            }
        }
    }
}

#[derive(Clone)]
pub enum TopLevelMsg {
    ChangeTab(Tabs),
    SetMsg(String),
    JoinGroupData{group_name: String, player_name: String},
    JoinGroup,
    LeftGroup,
    GroupsUpdated,
    EraseData,
    ImportData,
    FileSelected(web_sys::File),
    LoadFile(Vec<u8>),
    CancelModal,
    ModalOk,
    DoNothing,
}

impl From<std::option::NoneError> for TopLevelMsg {
    fn from(_: std::option::NoneError) -> Self {
        TopLevelMsg::DoNothing
    }
}

impl std::ops::Try for TopLevelMsg {
    type Ok = Self;
    type Error = Self;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        
        match self {
            TopLevelMsg::DoNothing => Err(TopLevelMsg::DoNothing),
            _ => Ok(self)
        }
    }
    fn from_error(_: Self::Error) -> Self {
        TopLevelMsg::DoNothing
    }
    fn from_ok(v: Self::Ok) -> Self {
        v
    }
    
}

impl From<GlobalReq> for TopLevelMsg {
    fn from(msg: GlobalReq) -> Self {
        match msg {
            GlobalReq::SetHeaderMsg(msg) => {
                TopLevelMsg::SetMsg(msg)
            }
            GlobalReq::JoinGroup => {
                TopLevelMsg::JoinGroup
            }
            GlobalReq::EraseData => {
                TopLevelMsg::EraseData
            }
            GlobalReq::ImportData => {
                TopLevelMsg::ImportData
            }
        }
    }
}

impl From<GroupFldrAgentOutMsg> for TopLevelMsg {
    fn from(msg: GroupFldrAgentOutMsg) -> Self {
        match msg {
            GroupFldrAgentOutMsg::JoinedGroup => {
                TopLevelMsg::DoNothing   
            }
            GroupFldrAgentOutMsg::LeftGroup => {
                TopLevelMsg::LeftGroup
            }
            GroupFldrAgentOutMsg::GroupUpdated => {
                TopLevelMsg::GroupsUpdated
            }
        }
    }
}



#[derive(PartialEq)]
pub enum ModalStatus {
    JoinGroup,
    EraseData,
    ImportData,
    Closed,
}

/// Root component
pub struct App
{
    active_tab: Tabs,
    link: ComponentLink<Self>,
    load_file_callback: Callback<ChangeData>,
    message_txt: String,
    message_clear_timeout_handle: Option<TimeoutTask>,
    message_clear_callback: Callback<()>,
    _producer: Box<dyn Bridge<GlobalMsgBus>>,
    group_folder: Box<dyn Bridge<GroupFldrMsgBus>>,
    modal_status: ModalStatus,
    in_group: bool,
    load_file_callback_promise: Option<ReaderTask>,
    file_input_ref: NodeRef,
    _save_interval_handle: Option<IntervalTask>,
}

fn save_interval_callback(_:()) -> TopLevelMsg {
    ChipLibrary::get_instance().save_data();
    TopLevelMsg::DoNothing
}

fn join_group_callback(_: MouseEvent) -> TopLevelMsg {
    let window = web_sys::window()?;
    let document = window.document()?;

    let group_name_element = document.get_element_by_id("group_name")?;
    let player_name_element = document.get_element_by_id("player_name")?;

    let group_name_input = group_name_element.dyn_ref::<web_sys::HtmlInputElement>()?;
    let player_name_input = player_name_element.dyn_ref::<web_sys::HtmlInputElement>()?;

    let group_name = group_name_input.value();
    let player_name = player_name_input.value();

    TopLevelMsg::JoinGroupData{player_name, group_name}

}

fn load_file_callback(e: ChangeData) -> TopLevelMsg {
    if let ChangeData::Files(files) = e {
        let file = files.item(0)?;
        TopLevelMsg::FileSelected(file)
    } else {
        TopLevelMsg::DoNothing
    }
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

        if message.is_empty() {
            self.message_clear_timeout_handle.take();
        } else {
            self.set_message_clear_timeout();
        }

        self.message_txt = message;
        true
    }

    fn set_message_clear_timeout(&mut self) {

        //ensure that previous timeout is cancelled
        self.message_clear_timeout_handle.take();
        let handle = TimeoutService::new().spawn(Duration::from_secs(15), self.message_clear_callback.clone());
        //let callback = self.link.callback_once(|_: ()| TopLevelMsg::SetMsg("".to_owned()));
        
        self.message_clear_timeout_handle = Some(handle);
        
    }

    fn gen_nav_tabs(&self) -> Html {

        match self.active_tab {

            Tabs::Library => {
                html! {
                    <div class="btn-group" role="tabs" style="padding-left: 125px; transform: translate(0px,6px)">
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Folder))>{"Folder"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Pack))>{"Pack"}</button>
                        <button class="btn activeNavTab">{"Library"}</button>
                    </div>
                }
            }
            Tabs::Pack => {
                html! {
                    <div class="btn-group" role="tabs" style="padding-left: 125px; transform: translate(0px,6px)">
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Folder))>{"Folder"}</button>
                        <button class="btn activeNavTab">{"Pack"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Library))>{"Library"}</button>
                    </div>
                }
            }
            Tabs::Folder => {
                html! {
                    <div class="btn-group" role="tabs" style="padding-left: 125px; transform: translate(0px,6px)">
                        <button class="btn activeNavTab">{"Folder"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Pack))>{"Pack"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Library))>{"Library"}</button>
                    </div>
                }
            }
            _ => { 
                unreachable!()
            }
        }
        
    }

    fn build_modal(&self) -> Html {
        match self.modal_status {

            ModalStatus::JoinGroup => {
                self.join_group_modal()
            }
            ModalStatus::EraseData => {
                self.import_or_erase_modal(false)
            }
            ModalStatus::ImportData => {
                self.import_or_erase_modal(true)
            }
            
            //closed, display nothing
            ModalStatus::Closed => html!{},
        }
    }

    fn join_group_modal(&self) -> Html {
        let cancel_callback = self.link.callback(|_: MouseEvent| TopLevelMsg::CancelModal);
        let ok_callback = self.link.callback(join_group_callback);

        html!{
            <div class="yew-modal">
                <div class="yew-modal-content">
                    <div class="yew-modal-header">
                        <h2>{"Join Group"}</h2>
                    </div>
                    <div class="yew-modal-body">
                        <input type="text" placeholder="group name" id="group_name"/>
                        <br/>
                        <input type="text" placeholder="player name" id="player_name"/>
                    </div>
                    <div class="yew-modal-footer">
                        <span style="padding-left: 5px">
                            <button class="btn btn-danger" onclick={ok_callback}>{"Ok"}</button>
                        </span>
                        <span style="float: right">
                            <button class="btn btn-secondary" onclick={cancel_callback}>{"Cancel"}</button>
                        </span>
                    </div>
                </div>
            </div>
        }

    }

    fn import_or_erase_modal(&self, import: bool) -> Html {
        let cancel_callback = self.link.callback(|_: MouseEvent| TopLevelMsg::CancelModal);
        let ok_callback = self.link.callback(|_:MouseEvent| TopLevelMsg::ModalOk);
        let header_text = if import {"Import Data"} else {"Erase Data"};
        html!{
            <div class="yew-modal">
                <div class="yew-modal-content">
                    <div class="yew-modal-header">
                        <h2>{header_text}</h2>
                    </div>
                    <div class="yew-modal-body">
                        {"This will erase all existing data, are you sure?"}
                    </div>
                    <div class="yew-modal-footer">
                        <span style="padding-left: 5px">
                            <button class="btn btn-danger" onclick={ok_callback}>{"Ok"}</button>
                        </span>
                        <span style="float: right">
                            <button class="btn btn-secondary" onclick={cancel_callback}>{"Cancel"}</button>
                        </span>
                    </div>
                </div>
            </div>
        }
        //todo!();
    }

    fn modal_ok(&mut self) -> bool {
        match self.modal_status {
            ModalStatus::EraseData => {
                ChipLibrary::get_instance().erase_data();
            }
            ModalStatus::ImportData => {
                if let Some(element) = self.file_input_ref.cast::<web_sys::HtmlInputElement>() {
                    element.click();
                }
                return false;
            }
            ModalStatus::Closed | ModalStatus::JoinGroup => {
                unreachable!();
            }
        }
        self.modal_status = ModalStatus::Closed;
        self.active_tab = Tabs::Library;
        true
    }

    fn load_file(&mut self, json: Vec<u8>) -> bool {
        self.load_file_callback_promise.take();
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("load file callback was called"));
            self.modal_status = ModalStatus::Closed;
            self.active_tab = Tabs::Library;
            let json = match String::from_utf8(json) {
                Ok(json) => json,
                Err(_) => {
                    unsafe{alert("File was invalid, corrupted maybe?")};
                    return false;
                }
            };
            match ChipLibrary::get_instance().import_json(json) {
                Ok(()) => {
                    self.set_message("chips imported".to_string());
                }
                Err(msg) => {
                    unsafe{alert(msg)};
                }
            }
        true
    }

    fn file_selected(&mut self, file: web_sys::File) -> bool {
        let callback = self.link.callback(|e: FileData|{
            TopLevelMsg::LoadFile(e.content)
        });
        let handle = match ReaderService::new().read_file(file, callback) {
            Ok(handle) => handle,
            Err(why) => {
                unsafe{alert(&why.to_string())};
                return false;
            }
        };
        self.load_file_callback_promise = Some(handle);
        false
    }
}

impl Component for App {
    type Message = TopLevelMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let global_callback = link.callback(|e: GlobalReq| {
            TopLevelMsg::from(e)
        });
        let _producer = GlobalMsgBus::bridge(global_callback);

        let group_callback = link.callback(|e: GroupFldrAgentOutMsg|{
            TopLevelMsg::from(e)
        });

        let group_folder = GroupFldrMsgBus::bridge(group_callback);

        let load_file_callback = link.callback(load_file_callback);

        let _save_interval_handle = if storage_available("localStorage".to_owned()) {
            let callback = link.callback(save_interval_callback);
            let handle = IntervalService::new().spawn(Duration::from_secs(300), callback);//set_interval(300000, save_interval_callback).unwrap();
            Some(handle)
        } else {
            None
        };

        let message_clear_callback = link.callback(|_:()|{
            TopLevelMsg::SetMsg(String::new())
        });

        App {
            active_tab: Tabs::Library,
            message_txt: String::new(),
            message_clear_timeout_handle: None,
            link,
            _producer,
            load_file_callback,
            modal_status: ModalStatus::Closed,
            in_group: false,
            load_file_callback_promise: None,
            file_input_ref: NodeRef::default(),
            _save_interval_handle,
            group_folder,
            message_clear_callback,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        return match msg {
            TopLevelMsg::ChangeTab(tab) => self.change_tab(tab),
            TopLevelMsg::SetMsg(message) => self.set_message(message),
            TopLevelMsg::JoinGroup => {
                self.modal_status = ModalStatus::JoinGroup;
                true
            }
            TopLevelMsg::EraseData => {
                self.modal_status = ModalStatus::EraseData;
                true
            }
            TopLevelMsg::ImportData => {
                self.modal_status = ModalStatus::ImportData;
                true
            }
            TopLevelMsg::CancelModal => {
                self.modal_status = ModalStatus::Closed;
                true
            }
            TopLevelMsg::ModalOk => {
                self.modal_ok()
            }
            TopLevelMsg::JoinGroupData{group_name, player_name} => {
                self.modal_status = ModalStatus::Closed;
                self.group_folder.send(GroupFldrAgentReq::JoinGroup{group_name, player_name});
                self.in_group = true;
                true
            }
            TopLevelMsg::LoadFile(json) => self.load_file(json),
            TopLevelMsg::FileSelected(file) => self.file_selected(file),
            TopLevelMsg::LeftGroup => {
                self.in_group = false;
                true
            },
            TopLevelMsg::GroupsUpdated => true,
            TopLevelMsg::DoNothing => false,
        }
    }

    fn view(&self) -> Html {

        //let set_msg_callback = self.link.callback(|msg: String| TopLevelMsg::SetMsg(msg));
        
        html!{
            <>
            <div style="background-color: #00637b; padding: 5px; max-width: 720px; margin: auto;">
                <div style="background-color: #ffbd18; font-family: Lucida Console; margin: 5px; color: #FFFFFF; font-weight: bold">
                    <span style="padding-left: 5px">{self.active_tab.to_display_text()}</span><span style="float: right; color: red">{&self.message_txt}</span>
                </div>
                <div style="background-color: #4abdb5; padding: 10px;">
                    {self.gen_nav_tabs()}
                    <div class="container-fluid">
                        <div class="row">
                            <Library active={self.active_tab == Tabs::Library}/>
                            <Pack active={self.active_tab == Tabs::Pack}/>
                            <Folder active={self.active_tab == Tabs::Folder} in_folder_group={self.in_group}/>
                            <ChipDescBox/>
                        </div>
                    </div>
                </div>
            </div>
            {self.build_modal()}
            <input id="jsonFile" type="file" style="display: none" accept=".json" onchange={self.load_file_callback.clone()} ref={self.file_input_ref.clone()}/>
            </>
        }

        //let library: RwLockReadGuard<ChipLibrary> = get_instance().get().unwrap().read().unwrap();
    }
}
