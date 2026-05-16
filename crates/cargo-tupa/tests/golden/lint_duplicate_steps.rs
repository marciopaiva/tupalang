pipeline! {
  name: DuplicateSteps,
  input: (),
  steps: [
    step("a"){1},
    step("a"){2}
  ],
  constraints: []
}
