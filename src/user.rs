use nix::unistd::Uid;

pub fn is_user_root() -> bool {
    Uid::effective().is_root()
}
