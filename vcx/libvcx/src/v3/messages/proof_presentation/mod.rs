pub mod presentation_proposal;
pub mod presentation_request;
pub mod presentation;

#[cfg(test)]
pub mod test {
    use v3::messages::ack;
    use v3::messages::error;
    use v3::messages::proof_presentation::presentation_request::tests::_presentation_request;

    pub fn _ack() -> ack::Ack {
        ack::tests::_ack().set_thread_id(_presentation_request().id.clone().0)
    }

    pub fn _problem_report() -> error::ProblemReport {
        error::tests::_problem_report().set_thread_id(_presentation_request().id.clone().0)
    }
}