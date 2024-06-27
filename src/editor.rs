use nih_plug::editor::ParentWindowHandle;
use nih_plug::prelude::Editor;
use nih_plug::context::gui::GuiContext;

use nih_plug_iced::*;
use std::sync::Arc;

pub(crate) fn create() -> Option<Box<dyn Editor>> {
    create_iced_editor::<GainEditor>(IcedState::from_size(200, 200), ())
}

struct GainEditor {

}

impl IcedEditor for GainEditor {
    type Executor = executor::Default;

    type Message = ();

    type InitializationFlags = ();

    fn new(
        _initialization_flags: Self::InitializationFlags,
        _context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        (GainEditor{}, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        todo!()
    }

    fn update(
        &mut self,
        window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        todo!()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        todo!()
    }
}
