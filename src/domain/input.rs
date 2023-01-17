pub enum Input {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Rotate(i32),
    ToggleFullscreen,
    ShowFps,
    Quit,
}
