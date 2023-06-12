//! A repo trait, for convenience

use diesel::Identifiable;

/// A crud repository
pub trait Repository<T, Id>
where
    for<'a> &'a T: Identifiable<Id = &'a Id>,
{
    /// Gets the number of entities available
    fn count(&mut self) -> u64;
    /// Deletes an object from the repository
    fn delete(&mut self, obj: T);
    fn delete_all(&mut self);
    fn delete_iter<I: IntoIterator<Item = T>>(&mut self, iter: I);
    fn delete_by_ids<I: IntoIterator<Item = Id>>(&mut self, iter: I);
    fn delete_by_id(&mut self, iter: Id);

    fn find_all(&mut self) -> Vec<T>;
    fn find_all_by_id<I: IntoIterator<Item = Id>>(&mut self, iter: I) -> Vec<T>;

    /// Finds an entity by an id
    fn find_by_id(&mut self, id: Id) -> Option<T>;

    /// Stores the object
    fn save(&mut self, obj: T);
    fn save_all<I: IntoIterator<Item = T>>(&mut self, obj: I);
}
