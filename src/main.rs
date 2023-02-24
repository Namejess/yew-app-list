use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
mod video;

//Struct pour la Video
#[derive(Clone, PartialEq, Deserialize)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

//Struct pour les propriétés de la liste des videos
#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

//Struct pour les propriétés des détails de la video
#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

//Composant fonctionnel pour les détails de la video
#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <p>{ video.speaker.clone() }</p>
            <video src={video.url.clone()} controls={true}></video>

        </div>
    }
}

//Composant fonctionnel pour la liste des videos
#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone(); //On clone le callback pour pouvoir l'utiliser dans la fonction
    videos
        .iter() // Iterator pour parcourir la liste
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };
            //On mappe les videos pour les afficher
            html! { //On retourne le html
                <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p> //On affiche le titre et le speaker
            }
        })
        .collect() //On collecte le tout dans un Html pour le retourner à la fin de la fonction app
}

//Composant App fonctionnel
#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    let selected_video = use_state(|| None); // On utilise le hook use_state pour stocker la video sélectionnée

    let on_video_select = {
        let selected_video = selected_video.clone(); //On clone le hook pour pouvoir l'utiliser dans la fonction
        Callback::from(move |video: Video| {
            selected_video.set(Some(video)); //On set la video sélectionnée
        })
    };

    let details = selected_video //On récupère la video sélectionnée
        .as_ref() //On la transforme en référence
        .map(|video| html! { <VideoDetails video={video.clone()} /> }); //On mappe la video pour l'afficher

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />//On passe la liste des videos et le callback pour la sélection
            </div>
            {for details} //On affiche les détails de la video sélectionnée
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
