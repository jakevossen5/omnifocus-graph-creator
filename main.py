import csv

def main():
    print("Starting main")
    getDataFromCsv()

def getDataFromCsv():
    with open('omnifocus-data/OmniFocus.csv') as csvDataFile:
        csvReader = csv.reader(csvDataFile)
        for row in csvReader:
            print(row)
    
    
class Task:
    def __init__(self, taskID, taskType, project, status, startDate, completionDate):
        self.taskID = taskID
        self.taskType = taskType
        self.project = project
        self.status = status
        self.startDate = startDate # TODO convert to datetime
        self.completionDate = completionDate
        
    
    
main()
