import csv

def main() -> None:
    getDataFromCsv()

def getDataFromCsv() -> list:
    tasks = []
    with open('omnifocus-data/OmniFocus.csv') as csvDataFile:
        csvReader = csv.reader(csvDataFile)
        for row in csvReader:
            tasks.append(Task(row[0], row[1], row[3], row[4], row[6], row[8]))
    return tasks

    
    
class Task:
    def __init__(self, taskID, taskType, status, project, startDate, completionDate) -> None:
        self.taskID = taskID
        self.taskType = taskType
        self.project = project
        self.status = status
        self.startDate = startDate # TODO convert to datetime
        self.completionDate = completionDate
    def __str__(self) -> str:
        return "ID: " + self.taskID + "\n taskType: " + self.taskType + "\n status: " + self.status + "\n project: " + self.project + "\n status: " + self.status + "\n completionDate: " + self.completionDate
    
    
main()
