#!/usr/bin/env python3

from sys import stdin
from select import select
from signal import signal, SIGINT, SIG_DFL

from PyQt5.QtCore import QTimer
from PyQt5.QtWidgets import QWidget, QApplication, QMainWindow
from PyQt5.QtWidgets import QGridLayout, QLabel

## QGridLayout
## from PyQt5.QtWidgets import QTabWidget, QPushButton, QFrame, QLabel
## from PyQt5.QtWidgets import QTableWidget, QTableWidgetItem

class Globals:
  def __init__(self):
    self.app = None
    self.mainWindow = None
    self.centralWidget = None
    self.tabWidget = None
    self.timer = None
    self.ma = None
    self.mb = None

g = Globals()

styleSheet = """
QWidget {
  background-color:#e0e0ff;
}
QLabel {
  qproperty-alignment: AlignRight;
  font-size:48px;
  max-height:60px;
  border:1px solid black;
  border-radius:5px;
}
QLabel.mmm { background-color:#ffc0c0;}
QLabel.vvv { background-color:#c0ffc0;}
"""

def timerTick():
  while select([stdin,],[],[],0.0)[0]:
    arr = stdin.readline().strip().split()
    print("XX {%s} {%s}" % (arr[0], arr[1]))
    if len(arr) < 2:
      continue;
    g.ma.setText(arr[0])
    g.mb.setText(arr[1])

def setupMainWindow():
  g.app = QApplication([])
  signal(SIGINT, SIG_DFL) ## Make Ctrl-C work
  g.mainWindow = QMainWindow()
  g.mainWindow.setWindowTitle("Encoder Test")
  g.mainWindow.setGeometry(1500, 800, 400, 400)
  g.centralWidget = QWidget()
  ## g.centralWidget.setStyleSheet(styleSheet)
  g.mainWindow.setCentralWidget(g.centralWidget)
  grid = QGridLayout()
  g.centralWidget.setLayout(grid)
  def mkLabel(x, y, cls, txt):
    lbl = QLabel(txt)
    lbl.setProperty("class", cls)
    grid.addWidget(lbl, x, y)
    return lbl
  m00  = mkLabel(0, 0, "mmm", "Motor A:")
  g.ma = mkLabel(0, 1, "vvv", "0")
  m01  = mkLabel(1, 0, "mmm", "Motor B:")
  g.mb = mkLabel(1, 1, "vvv", "0")
  g.mainWindow.setStyleSheet(styleSheet)
  g.mainWindow.show()

def setupTimer():
  g.timer = QTimer()
  g.timer.setInterval(100)
  g.timer.timeout.connect(timerTick)
  g.timer.start()

def main():
  setupMainWindow()
  setupTimer()
  g.app.exec_()

main()
