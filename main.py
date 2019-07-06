import datetime
from datetime import timezone
import csv

def main() -> None:
    tasks = getDataFromCsv()
    x = 0
    for t in tasks:
        if (t.completionDate != "" and ((datetime.datetime.now(timezone.utc) - t.completionDate) < datetime.timedelta(days=7))):
            print(t)
            x = x + 1
    print("Total: " + str(x))

def getDataFromCsv() -> list:
    tasks = []
    with open('omnifocus-data/OmniFocus.csv') as csvDataFile:
        csvReader = csv.reader(csvDataFile)
        for row in csvReader:
            if (row[0] != "Task ID" and row[1] != "Inbox"): # This is to prevent getting the header row into the data
                tasks.append(Task(row[0], row[1], row[2], row[3], row[4], row[6], row[8]))
    return tasks

    
    
class Task:
    def __init__(self, taskID, taskType, name, status, project, startDate, completionDate) -> None:
        self.taskID = taskID
        self.taskType = taskType
        self.name = name
        self.project = project
        self.status = status
        self.startDate = startDate # TODO convert to datetime
        self.completionDate = datetime.datetime.strptime(completionDate, '%Y-%m-%d %H:%M:%S %z') if completionDate != "" else ""
    def __str__(self) -> str:
        return " ID: " + self.taskID + "\n taskType: " + self.taskType + "\n name: " + self.name + "\n status: " + self.status + "\n project: " + self.project + "\n status: " + self.status + "\n completionDate: " + str(self.completionDate)
    
    
main()
