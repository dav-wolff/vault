use std::{borrow::Cow, collections::VecDeque, time::Duration};

use leptos::*;
use stylance::{classes, import_style};

import_style!(style, "notify.css");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NotificationKind {
	Info,
	Error,
}

#[derive(Clone, Debug)]
struct NotificationData {
	id: i32,
	kind: NotificationKind,
	message: Cow<'static, str>,
}

#[derive(Clone, Copy, Debug)]
pub struct Notify(WriteSignal<VecDeque<NotificationData>>);

impl Notify {
	pub fn from_context() -> Self {
		use_context().unwrap()
	}
	
	fn add_notification(&self, kind: NotificationKind, message: Cow<'static, str>) {
		self.0.update(|notifications| {
			let id = notifications.back()
				.map(|notification| notification.id + 1)
				.unwrap_or_default();
			
			notifications.push_back(NotificationData {
				id,
				kind,
				message,
			});
		});
	}
	
	pub fn info(&self, message: impl Into<Cow<'static, str>>) {
		self.add_notification(NotificationKind::Info, message.into())
	}
	
	pub fn error(&self, message: impl Into<Cow<'static, str>>) {
		self.add_notification(NotificationKind::Error, message.into())
	}
}

#[component]
pub fn NotifyProvider(children: Children) -> impl IntoView {
	let (notifications, set_notifications) = create_signal(VecDeque::new());
	
	provide_context(Notify(set_notifications));
	
	let delete_notification = move |id| {
		set_notifications.update(|notifications| {
			if let Some(index) = notifications.iter()
				.position(|notification| notification.id == id)
			{
				notifications.remove(index);
			}
		});
	};
	
	view! {
		{children()}
		<div class=style::container>
			<For
				each=move || notifications()
				key=move |notification| notification.id
				children=move |notification| view! {
					<Notification
						data=notification.clone()
						deleter=move || delete_notification(notification.id)
					/>
				}
			/>
		</div>
	}
}

#[component]
fn Notification<D>(
	data: NotificationData,
	deleter: D
) -> impl IntoView
where
	D: Fn() + Copy + 'static
{
	let NotificationData {kind, message, ..} = data;
	
	let (is_fading, set_fading) = create_signal(false);
	
	create_effect(move |_| {
		set_timeout(move || set_fading(true), Duration::from_millis(3_000));
	});
	
	let notification_class = match kind {
		NotificationKind::Info => style::info,
		NotificationKind::Error => style::error,
	};
	
	view! {
		<div class=move || classes!(style::notification, is_fading().then_some(style::fading))>
			<p
				class=notification_class
				on:animationend=move |_| deleter()
			>
				{message}
			</p>
		</div>
	}
}
