use crate::{
    AppContext, AsyncAppContext, Context, Effect, EntityId, EventEmitter, Handle, Reference,
    Subscription, Task, WeakHandle,
};
use std::{future::Future, marker::PhantomData};

pub struct ModelContext<'a, T> {
    app: Reference<'a, AppContext>,
    entity_type: PhantomData<T>,
    entity_id: EntityId,
}

impl<'a, T: Send + Sync + 'static> ModelContext<'a, T> {
    pub(crate) fn mutable(app: &'a mut AppContext, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Mutable(app),
            entity_type: PhantomData,
            entity_id,
        }
    }

    pub fn handle(&self) -> WeakHandle<T> {
        self.app.entities.weak_handle(self.entity_id)
    }

    pub fn observe<E: Send + Sync + 'static>(
        &mut self,
        handle: &Handle<E>,
        on_notify: impl Fn(&mut T, Handle<E>, &mut ModelContext<'_, T>) + Send + Sync + 'static,
    ) -> Subscription {
        let this = self.handle();
        let handle = handle.downgrade();
        self.app.observers.insert(
            handle.id,
            Box::new(move |cx| {
                if let Some((this, handle)) = this.upgrade().zip(handle.upgrade()) {
                    this.update(cx, |this, cx| on_notify(this, handle, cx));
                    true
                } else {
                    false
                }
            }),
        )
    }

    pub fn subscribe<E: EventEmitter + Send + Sync + 'static>(
        &mut self,
        handle: &Handle<E>,
        on_event: impl Fn(&mut T, Handle<E>, &E::Event, &mut ModelContext<'_, T>)
            + Send
            + Sync
            + 'static,
    ) -> Subscription {
        let this = self.handle();
        let handle = handle.downgrade();
        self.app.event_handlers.insert(
            handle.id,
            Box::new(move |event, cx| {
                let event = event.downcast_ref().expect("invalid event type");
                if let Some((this, handle)) = this.upgrade().zip(handle.upgrade()) {
                    this.update(cx, |this, cx| on_event(this, handle, event, cx));
                    true
                } else {
                    false
                }
            }),
        )
    }

    pub fn on_release(
        &mut self,
        on_release: impl Fn(&mut T, &mut AppContext) + Send + Sync + 'static,
    ) -> Subscription {
        self.app.release_handlers.insert(
            self.entity_id,
            Box::new(move |this, cx| {
                let this = this.downcast_mut().expect("invalid entity type");
                on_release(this, cx);
            }),
        )
    }

    pub fn observe_release<E: Send + Sync + 'static>(
        &mut self,
        handle: &Handle<E>,
        on_release: impl Fn(&mut T, &mut E, &mut ModelContext<'_, T>) + Send + Sync + 'static,
    ) -> Subscription {
        let this = self.handle();
        self.app.release_handlers.insert(
            handle.id,
            Box::new(move |entity, cx| {
                let entity = entity.downcast_mut().expect("invalid entity type");
                if let Some(this) = this.upgrade() {
                    this.update(cx, |this, cx| on_release(this, entity, cx));
                }
            }),
        )
    }

    pub fn notify(&mut self) {
        self.app.push_effect(Effect::Notify {
            emitter: self.entity_id,
        });
    }

    pub fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: 'static + Send + Sync,
    {
        let mut global = self.app.pop_global::<G>();
        let result = f(global.as_mut(), self);
        self.app.push_global(global);
        result
    }

    pub fn spawn<Fut, R>(
        &self,
        f: impl FnOnce(WeakHandle<T>, AsyncAppContext) -> Fut + Send + 'static,
    ) -> Task<R>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let this = self.handle();
        self.app.spawn(|cx| f(this, cx))
    }
}

impl<'a, T: EventEmitter + Send + Sync + 'static> ModelContext<'a, T> {
    pub fn emit(&mut self, event: T::Event) {
        self.app.push_effect(Effect::Emit {
            emitter: self.entity_id,
            event: Box::new(event),
        });
    }
}

impl<'a, T: 'static> Context for ModelContext<'a, T> {
    type EntityContext<'b, 'c, U: Send + Sync + 'static> = ModelContext<'b, U>;
    type Result<U> = U;

    fn entity<U: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, U>) -> U,
    ) -> Handle<U> {
        self.app.entity(build_entity)
    }

    fn update_entity<U: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<U>,
        update: impl FnOnce(&mut U, &mut Self::EntityContext<'_, '_, U>) -> R,
    ) -> R {
        self.app.update_entity(handle, update)
    }
}