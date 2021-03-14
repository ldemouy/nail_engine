pub trait Exit {
    fn is_locked(&self) -> bool;
    fn lock(&mut self);
    fn unlock(&mut self);
    fn leads_to(&self) -> String;
}

pub trait Item {
    fn get_name(&self) -> String;
}

pub trait Room<E, I>
where
    E: Exit,
    I: Item,
{
    fn get_contents(&self) -> Vec<I>;
    fn get_exits(&self) -> Vec<E>;
    fn get_name(&self) -> String;
}

pub trait Player<I>
where
    I: Item,
{
    fn get_name(&self) -> String;
    fn get_inventory(&self) -> Vec<I>;
}

pub trait Listener {
    fn get_receiver(&self) -> &crossbeam::channel::Receiver<Option<nail_common::Message>>;
    fn get_sender(&self) -> &crossbeam::channel::Sender<Option<nail_common::Message>>;
}
