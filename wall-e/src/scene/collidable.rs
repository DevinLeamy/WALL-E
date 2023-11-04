// CollidableClone allows the trait object dyn Collidable to derive Clone.
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object

pub trait Collidable: CollidableClone + Send + std::fmt::Debug {}

pub trait CollidableClone {
    fn clone_collidable(&self) -> Box<dyn Collidable>;
}

impl<T> CollidableClone for T
where
    T: 'static + Collidable + Clone,
{
    fn clone_collidable(&self) -> Box<dyn Collidable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Collidable> {
    fn clone(&self) -> Box<dyn Collidable> {
        self.clone_collidable()
    }
}
