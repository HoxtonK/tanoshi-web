use super::component::Manga;
use serde::Deserialize;
use yew::format::{Json, Nothing, Text};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use super::component::model::{FavoriteManga, GetFavoritesResponse, GetMangasResponse, MangaModel};
use super::component::{BottomBar, TopBar};

use http::{Request, Response};
use std::borrow::BorrowMut;
use yew::services::storage::Area;
use yew::services::StorageService;
use yew::utils::{document, window};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Properties)]
pub struct Props {
    pub source: String,
}

pub struct Catalogue {
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    source: String,
    page: i32,
    mangas: Vec<MangaModel>,
    favorites: Vec<String>,
    is_fetching: bool,
    token: String,
    closure: Closure<dyn Fn()>,
}

pub enum Msg {
    FetchReady(String),
    ScrolledDown,
    Noop,
}

impl Component for Catalogue {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let token = {
            if let Ok(token) = storage.restore("token") {
                token
            } else {
                "".to_string()
            }
        };
        let tmp_link = link.clone();
        let closure = Closure::wrap(Box::new(move || {
            let current_scroll = window().scroll_y().expect("error get scroll y")
                + window().inner_height().unwrap().as_f64().unwrap();
            let height = document().body().unwrap().offset_height() as f64;
            if current_scroll >= height {
                tmp_link.send_message(Msg::ScrolledDown);
            }
        }) as Box<dyn Fn()>);
        Catalogue {
            fetch_task: None,
            link,
            source: props.source,
            page: 1,
            mangas: vec![],
            favorites: vec![],
            is_fetching: false,
            token,
            closure,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.fetch_catalogue();
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(data) => self.scrape_catalogue(data),
            Msg::ScrolledDown => {
                if !self.is_fetching {
                    self.page += 1;
                }
            }

            Msg::Noop => {
                info!("noop");
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
            <TopBar />
            <BottomBar/>
            <div class="container-fluid">
                <div class="row row-cols-sm-2 row-cols-md-3 row-cols-lg-5 row-cols-xl-6" style="height: 100%;">
                { for self.mangas.iter().map(|manga| html!{
                <Manga
                    title=manga.title.to_owned()
                    thumbnail=manga.thumbnail_url.to_owned()
                    path=manga.path.to_owned()
                    source=self.source.to_owned()
                    is_favorite={if self.favorites.contains(&manga.title.to_owned()){true} else {false}}/>
                }) }
                </div>
            </div>
            </>
        }
    }

    fn destroy(&mut self) {
        window().set_onscroll(None);
    }
}

impl Catalogue {
    fn fetch_catalogue(&mut self) {
        let params = vec![
            ("keyword".to_owned(), "".to_owned()),
            ("page".to_owned(), "1".to_owned()),
            ("sortBy".to_owned(), "popularity".to_owned()),
            ("sortOrder".to_owned(), "descending".to_owned()),
        ];

        let urlencoded = serde_urlencoded::to_string(params).unwrap();

        let req = Request::post(format!(
            "https://api.allorigins.win/raw?url=https://mangaseeonline.us/search/request.php"
        ))
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=utf-8",
        )
        .body(Ok(urlencoded))
        .expect("failed to build request");

        if let Ok(task) = FetchService::new().fetch(
            req,
            self.link.callback(|response: Response<Text>| {
                if let (meta, Ok(data)) = response.into_parts() {
                    return Msg::FetchReady(data);
                }
                Msg::Noop
            }),
        ) {
            self.fetch_task = Some(FetchTask::from(task));
        }
    }

    fn scrape_catalogue(&mut self, html: String) {
        let mut mangas: Vec<MangaModel> = Vec::new();

        let document = scraper::Html::parse_document(&html);

        let selector = scraper::Selector::parse(".requested .row").unwrap();
        for row in document.select(&selector) {
            let mut manga = MangaModel {
                title: String::from(""),
                author: String::from(""),
                genre: vec![],
                status: String::from(""),
                description: String::from(""),
                path: String::from(""),
                thumbnail_url: String::from(""),
            };

            let sel = scraper::Selector::parse("img").unwrap();
            for el in row.select(&sel) {
                manga.thumbnail_url = el.value().attr("src").unwrap().to_owned();
            }

            let sel = scraper::Selector::parse(".resultLink").unwrap();
            for el in row.select(&sel) {
                manga.title = el.inner_html();
                manga.path = el.value().attr("href").unwrap().to_owned();
            }
            mangas.push(manga);
        }

        self.mangas = mangas
    }
}
