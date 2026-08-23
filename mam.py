from tkinter import *

def create_notice(ntc):
    text_var = ntc.get()
    ntk = Tk()
    Label(ntk, text=text_var, font=("Arial", 72)).pack()
    ntk.mainloop()

win = Tk()
win.geometry("200x100")
win.title("Notice Creator")

notice_entry = Entry(width=20)
notice_entry.pack()
Button(text="Создать Заметку", command=lambda: create_notice(notice_entry)).pack()

win.mainloop()