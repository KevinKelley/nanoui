
//[Exposed=(Window,Worker)]
//interface EventTarget {
//  void addEventListener(DOMString type, EventListener? callback, optional boolean capture = false);
//  void removeEventListener(DOMString type, EventListener? callback, optional boolean capture = false);
//  boolean dispatchEvent(Event event);
//};
//
//callback interface EventListener {
//  void handleEvent(Event event);
//};

trait EventTarget {
	fn addEventListener(&self, typ: DomString, callback: Option<EventListener>, capture: Option<bool> /*=false*/);
	fn removeEventListener(&self, typ: DomString, callback: Option<EventListener>, capture: Option<bool> /*=false*/);
	fn dispatchEvent(&self, event: Event);
}
trait EventListener {
	fn handleEvent(&self, event: Event);
}


//[Constructor(DOMString type, optional EventInit eventInitDict), Exposed=(Window,Worker)]
//interface Event {
//  readonly attribute DOMString type;
//  readonly attribute EventTarget? target;
//  readonly attribute EventTarget? currentTarget;
//
//  const unsigned short NONE = 0;
//  const unsigned short CAPTURING_PHASE = 1;
//  const unsigned short AT_TARGET = 2;
//  const unsigned short BUBBLING_PHASE = 3;
//  readonly attribute unsigned short eventPhase;
//
//  void stopPropagation();
//  void stopImmediatePropagation();
//
//  readonly attribute boolean bubbles;
//  readonly attribute boolean cancelable;
//  void preventDefault();
//  readonly attribute boolean defaultPrevented;
//
//  [Unforgeable] readonly attribute boolean isTrusted;
//  readonly attribute DOMTimeStamp timeStamp;
//
//  void initEvent(DOMString type, boolean bubbles, boolean cancelable);
//};
//
//dictionary EventInit {
//  boolean bubbles = false;
//  boolean cancelable = false;
//};

enum EventPhase {
	NONE,
	CAPTURING_PHASE,
	AT_TARGET,
	BUBBLING_PHASE
}
struct Event {
	/*readonly*/ typ: DomString,
	/*readonly*/ target: Option<EventTarget>,
	/*readonly*/ currentTarget: Option<EventTarget>,
	eventPhase: EventPhase,
	bubbles: bool,
	cancelable: bool,
	isTrusted: bool,
	timeStamp: DOMTimeStamp
}
impl Event {
	fn stopPropagation(&self);
	fn stopImmediatePropagation(&self);
	fn preventDefault(&self);
	fn initEvent(&self, typ: DomString, bubbles:bool, cancelable:bool);
}