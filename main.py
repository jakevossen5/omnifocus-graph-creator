import datetime
from datetime import timezone
import csv
import matplotlib.pyplot as plt

def main() -> None:
    tasks = get_data_from_csv()
    x = 0
    tasks_per_days_ago = {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0}
    now_no_tz = datetime.datetime.now()
    delta_start_of_day = now_no_tz - datetime.datetime(now_no_tz.year, now_no_tz.month, now_no_tz.day, 0, 0, 0, 0)
    # print("Delta start of day:", delta_start_of_day)
    # print("Adding to delta:", delta_start_of_day + datetime.timedelta(days = 1))
    for t in tasks:
        for i in range(8):
            if (type(t.completionDate) == type(datetime.datetime.now(timezone.utc))):
                time_difference = datetime.datetime.now(timezone.utc) - t.completionDate
                # print("time_difference: ", time_difference)
                # print("First time difference:", delta_start_of_day + datetime.timedelta(days=i))
                # print("Second time difference:", (delta_start_of_day + datetime.timedelta(days=i + 1)))
                if ((time_difference < delta_start_of_day + datetime.timedelta(days=i)) and (time_difference > delta_start_of_day + datetime.timedelta(days=i - 1))):
                    tasks_per_days_ago[i] += 1
                    print(t)
    print("Totals: " + str(tasks_per_days_ago))

    create_plots(tasks_per_days_ago)

def create_plots(tasks_per_days_ago) -> None:
    x = list(range(8))
    y = []
    for i in x:
        y.append(tasks_per_days_ago[i])
    plt.scatter(x, y)
    plt.show()
    
    
def get_data_from_csv() -> list:
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
