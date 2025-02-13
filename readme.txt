this setup is inspired by gilles castel (https://castel.dev).

when i press super+s, rofi shows all my semesters.
`rofi -modi 'semesters:school-setup semesters' -show semesters -p '' -theme-str 'prompt \{ enabled: false; \}'`

when i press super+c, rofi shows all of my courses in my currently selected semester.
`rofi -modi 'courses:school-setup courses' -show courses -p '' -theme-str 'prompt \{ enabled: false; \}'`

when i press super+p it opens in my terminal.
`alacritty --working-directory $HOME/school/course`
