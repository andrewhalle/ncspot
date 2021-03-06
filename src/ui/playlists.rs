use std::sync::Arc;

use cursive::traits::Identifiable;
use cursive::view::ViewWrapper;
use cursive::views::IdView;

use playlists::{Playlist, Playlists};
use queue::Queue;
use ui::listview::ListView;

pub struct PlaylistView {
    list: IdView<ListView<Playlist>>,
}

impl PlaylistView {
    pub fn new(playlists: &Playlists, queue: Arc<Queue>) -> PlaylistView {
        let list = ListView::new(playlists.store.clone(), queue).with_id("list");

        PlaylistView { list: list }
    }
}

impl ViewWrapper for PlaylistView {
    wrap_impl!(self.list: IdView<ListView<Playlist>>);
}
