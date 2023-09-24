pub enum Input {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Rotate(i32),
    Action,
    ToggleFullscreen,
    ShowFps,
    Shoot,
    Quit,
}
