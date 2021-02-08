use std::any::{type_name, Any, TypeId};

use fxhash::FxHashMap;

use crate::component::Component;

/// The struct `Resources` represents a global resources. It is used to insert and call
/// global elements like services.
///
/// # Examples
///
/// ```
/// use dces::resources::Resources;
///
/// struct HelloResource;
///
/// impl HelloResource {
///     pub fn say_hello(&self) -> &str {
///         "Hello"
///     }    
/// }
///
/// let mut resources = Resources::new();
/// resources.insert(HelloResource);
///
/// assert_eq!("Hello", resources.get::<HelloResource>().say_hello());
/// ```
#[derive(Default)]
pub struct Resources {
    resources: FxHashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    /// Creates a service resources with an empty Resources map.
    pub fn new() -> Self {
        Resources::default()
    }

    /// Inserts a new resource.
    pub fn insert<C: Component>(&mut self, service: C) {
        self.resources.insert(TypeId::of::<C>(), Box::new(service));
    }

    /// Gets an element from the resources.
    ///
    /// # Panics
    ///
    /// Panics if the there is no element for the given type.
    pub fn get<C: Component>(&self) -> &C {
        self.resources
            .get(&TypeId::of::<C>())
            .unwrap_or_else(|| {
                panic!(
                    "Resources.get(): type {} could not be found.",
                    type_name::<C>()
                )
            })
            .downcast_ref()
            .unwrap_or_else(|| {
                panic!(
                    "Resources.get(): cannot convert to type: {}",
                    type_name::<C>()
                )
            })
    }

    /// Gets a mutable reference of the requested element.
    ///
    /// # Panics
    ///
    /// Panics if the there is no service for the given type.
    pub fn get_mut<C: Component>(&mut self) -> &mut C {
        self.resources
            .get_mut(&TypeId::of::<C>())
            .unwrap_or_else(|| {
                panic!(
                    "Resources.get(): type {} could not be found.",
                    type_name::<C>()
                )
            })
            .downcast_mut()
            .unwrap_or_else(|| {
                panic!(
                    "Resources.get(): cannot convert to type: {}",
                    type_name::<C>()
                )
            })
    }

    /// Try to get an element from the resources.
    pub fn try_get<C: Component>(&self) -> Option<&C> {
        if let Some(e) = self.resources.get(&TypeId::of::<C>()) {
            if let Some(r) = e.downcast_ref() {
                return Some(r);
            }
        }

        None
    }

    /// Try to get an element from the resources.
    pub fn try_get_mut<C: Component>(&mut self) -> Option<&mut C> {
        if let Some(e) = self.resources.get_mut(&TypeId::of::<C>()) {
            if let Some(r) = e.downcast_mut() {
                return Some(r);
            }
        }

        None
    }

    /// Returns `true` if the resources contains a resource of the given type overwise `false` .
    pub fn contains<C: Component>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<C>())
    }

    /// Returns the number of elements in the resources.
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns true if the resources contains no elements.
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ServiceOne;
    struct ServiceTwo;

    #[test]
    fn insert() {
        let mut resources = Resources::new();
        resources.insert(ServiceOne);
        resources.insert(ServiceTwo);

        assert!(resources.try_get::<ServiceOne>().is_some());
        assert!(resources.try_get::<ServiceTwo>().is_some());
    }

    #[test]
    fn try_get_mut() {
        let mut resources = Resources::new();
        resources.insert(ServiceOne);
        resources.insert(ServiceTwo);

        assert!(resources.try_get_mut::<ServiceOne>().is_some());
        assert!(resources.try_get_mut::<ServiceTwo>().is_some());
    }

    #[test]
    fn contains() {
        let mut resources = Resources::new();
        resources.insert(ServiceOne);

        assert!(resources.contains::<ServiceOne>());
        assert!(!resources.contains::<ServiceTwo>());
    }

    #[test]
    fn len() {
        let mut resources = Resources::new();
        assert_eq!(resources.len(), 0);

        resources.insert(ServiceOne);
        assert_eq!(resources.len(), 1);

        resources.insert(ServiceTwo);
        assert_eq!(resources.len(), 2);
    }

    #[test]
    fn is_empty() {
        let mut resources = Resources::new();
        assert!(resources.is_empty());

        resources.insert(ServiceOne);
        assert!(!resources.is_empty());
    }
}
