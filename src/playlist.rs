use url::Url;
use client::Client;
use error::{Error, Result};
use track::Track;
use user::User;

#[derive(Debug, Clone, Deserialize)]
pub enum PlaylistKind {
    #[serde(rename = "playlist")]
    Playlist
}

#[derive(Debug, Clone, Deserialize)]
pub enum PlaylistSharing {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Playlist {
    pub duration: u64,
    pub release_day: Option<i32>,
    pub permalink_url: String,
    pub permalink: String,
    pub purchase_url: Option<String>,
    pub description: Option<String>,
    pub uri: String,
    pub track_count: u64,
    pub user_id: u64,
    pub kind: PlaylistKind,
    pub title: String,
    pub id: u64,
    #[serde(default)]
    pub tracks: Option<Vec<Track>>,
    pub user: User,
}

#[derive(Debug)]
pub struct PlaylistRequestBuilder<'a> {
    client: &'a Client,
    query: Option<String>,
}

#[derive(Debug)]
pub struct SinglePlaylistRequestBuilder<'a> {
    client: &'a Client,
    pub id: usize,
}

impl<'a> SinglePlaylistRequestBuilder<'a> {
    /// Constructs a new track request.
    pub fn new(client: &'a Client, id: usize) -> SinglePlaylistRequestBuilder {
        SinglePlaylistRequestBuilder {
            client: client,
            id: id,
        }
    }

    /// Sends the request and return the tracks.
    pub fn get(&mut self) -> Result<Playlist> {
        let no_params: Option<&[(&str, &str)]> = None;
        let response = try!(self.client.get(&format!("/playlists/{}", self.id), no_params));
        let track: Playlist = try!(serde_json::from_reader(response));

        Ok(track)
    }

    pub fn request_url(&self) -> Url {
        let url = Url::parse(&format!("https://{}/playlists/{}", super::API_HOST, self.id)).unwrap();

        url
    }
}

impl<'a> PlaylistRequestBuilder<'a> {
    /// Creates a new playlist request builder, with no set parameters.
    pub fn new(client: &'a Client) -> Self {
        PlaylistRequestBuilder {
            client: client,
            query: None,
        }
    }

    /// Sets the search query filter, which will only return playlists with a matching query.
    pub fn query<S>(&'a mut self, query: S) -> &mut Self
        where S: AsRef<str> {
        self.query = Some(query.as_ref().to_owned());
        self
    }

    /// Returns a builder for a single playlist.
    pub fn id(&'a mut self, id: usize) -> SinglePlaylistRequestBuilder {
        SinglePlaylistRequestBuilder {
            client: &self.client,
            id: id,
        }
    }

    /// Performs the request and returns a list of playlists if there are any results, None otherwise,
    /// or an error if one occurred.
    pub fn get(&mut self) -> Result<Option<Vec<Playlist>>> {
        use serde_json::Value;

        let response = try!(self.client.get("/playlists", Some(self.request_params())));
        let playlist_list: Value = try!(serde_json::from_reader(response));

        if let Some(playlist_list) = playlist_list.as_array() {
            if playlist_list.is_empty() {
                return Ok(None);
            } else {
                let playlists: Vec<Playlist> = playlist_list
                    .iter().map(|p| serde_json::from_value::<Playlist>(p.clone()).unwrap()).collect();

                return Ok(Some(playlists));
            }
        }

        return Err(Error::ApiError("expected response to be an array".to_owned()));
    }

    fn request_params(&self) -> Vec<(&str, String)> {
        let mut result = vec![];

        if let Some(ref query) = self.query {
            result.push(("q", query.clone()));
        }

        result
    }
}