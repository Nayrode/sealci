//use crate::proto::controller as proto;
use crate::proto::scheduler as proto;

/// A struct representing an action in the queue.
/// The action has an ID, a score, and additional fields from the ActionRequest proto.
#[derive(Debug)]
pub(crate) struct Action {
    action_id: u32,
    context: proto::ExecutionContext,
    commands: Vec<String>,
    repo_url: String,
}

impl Action {
    /// Constructor
    pub fn new(action_id: u32, context: proto::ExecutionContext, commands: Vec<String>, repo_url: String) -> Self {
        Self {
            action_id,
            context,
            commands,
            repo_url,
        }
    }

    /// Action ID getter
    pub(crate) fn get_action_id(&self) -> u32 {
        self.action_id
    }

    /// Context getter
    pub(crate) fn _get_context(&self) -> &proto::ExecutionContext {
        &self.context
    }

    /// Runner type getter
    pub(crate) fn get_runner_type(&self) -> i32 {
        self.context.r#type
    }

    /// Container image getter
    pub(crate) fn get_container_image(&self) -> &str {
        match &self.context.container_image {
            Some(image) => image.as_str(),
            None => "",
        }
    }

    /// Commands getter
    pub(crate) fn get_commands(&self) -> &[String] {
        &self.commands
    }

    /// Repo URL getter
    pub(crate) fn get_repo_url(&self) -> &String {
        &self.repo_url
    }

    /// Action ID setter
    pub(crate) fn _set_action_id(&mut self, action_id: u32) {
        self.action_id = action_id;
    }

    /// Context setter
    pub(crate) fn _set_context(&mut self, context: proto::ExecutionContext) {
        self.context = context;
    }

    /// Runner type setter
    pub(crate) fn _set_runner_type(&mut self, runner_type: i32) {
        self.context.r#type = runner_type;
    }

    /// Container image setter
    pub(crate) fn _set_container_image(&mut self, container_image: String) {
        self.context.container_image = Some(container_image);
    }

    /// Commands setter
    pub(crate) fn _set_commands(&mut self, commands: Vec<String>) {
        self.commands = commands;
    }

    /// Repo URL setter
    pub(crate) fn _set_repo_url(&mut self, repo_url: String) {
        self.repo_url = repo_url;
    }

}

/// ActionsQueue is a collection of Actions stored in a vector.
/// The vector is sorted whenever necessary to maintain order.
pub struct ActionsQueue {
    actions: Vec<Action>,
}

impl ActionsQueue {
    /// Constructor
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    /// Insert an Action into the Action Queue and sort the Queue by score.
    pub(crate) fn _push(&mut self, item: Action) {
        self.actions.push(item);
    }

    /// Remove and return the Action with the lowest score (that is, the first Action), or return None if the Queue is empty.
    pub(crate) fn _pop(&mut self) -> Option<Action> {
        if self.actions.is_empty() {
            None
        } else {
            Some(self.actions.remove(0))
        }
    }

    /// Return the number of Actions in the Queue
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Check if the Action Queue is empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
    
}
