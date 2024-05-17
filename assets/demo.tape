# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.

Output assets/demo.gif

Hide
    Set Shell bash

    Set FontSize 18
    Set Width 1200
    Set Height 600

    Set Padding 10
    Set FontFamily "Ubuntu Mono Ligaturized"

    Set WindowBar Colorful
    Set WindowBarSize 40

    Set Theme "Dracula"
    Set TypingSpeed 70ms
Show

Sleep 500ms
Type "flawz --feeds 2024 --query xz"
Sleep 1s
Enter
Sleep 2s
Set TypingSpeed 400ms
Down 2
Sleep 1s
Enter
Sleep 1s
Set TypingSpeed 200ms
Down 10
Sleep 2s
Type "q"
Sleep 2s

Hide
    Type "q"
    Ctrl+D
Show
