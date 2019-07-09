#!/usr/local/bin/python3
import datetime
from datetime import timezone
import csv
import matplotlib.pyplot as plt
from sys import argv
import argparse
parser = argparse.ArgumentParser()
def main() -> None:
    parser.add_argument("-p", "--path-to-csv", dest="path_to_csv", default="OmniFocus.csv", help="Path to your OmniFocus CSV export default is 'OmniFocus.csv'")
    parser.add_argument("-d", "--days", dest="days", default=7, help="The number of days you want to view. Default is 7", type=int)
    parser.add_argument("-g", "--goal", dest="goal", default=10, help="Goal number of tasks for each day. Draws horizontal line on graph", type=int)
    args = parser.parse_args()
    path_to_csv = args.path_to_csv
    days = args.days + 1 # add 1 to be inclusive
    goal = args.goal

    tasks = get_data_from_csv(path_to_csv)

    tasks_per_days_ago = { }
    for i in range(days): # Do this as a lambda somehow?
        tasks_per_days_ago[i] = 0

    now_no_tz = datetime.datetime.now()
    delta_start_of_day = now_no_tz - datetime.datetime(now_no_tz.year, now_no_tz.month, now_no_tz.day, 0, 0, 0, 0)
    for t in tasks:
        for i in range(days):
            if (type(t.completionDate) == type(datetime.datetime.now(timezone.utc))):
                time_difference = datetime.datetime.now(timezone.utc) - t.completionDate
                if ((time_difference < delta_start_of_day + datetime.timedelta(days=i)) and (time_difference > delta_start_of_day + datetime.timedelta(days=i - 1))):
                    tasks_per_days_ago[i] += 1
                    # print(t)

    print("Totals: " + str(tasks_per_days_ago))

    create_plots(tasks_per_days_ago, days, goal)

def create_plots(tasks_per_days_ago, days, goal) -> None:
    # y = []
    for i in list(range(days)):
        color = 'b' if tasks_per_days_ago[i] >= goal else 'r'
        plt.scatter(i, tasks_per_days_ago[i], color = color)
        # y.append(tasks_per_days_ago[i])
    plt.axhline(y=goal, color='r', linestyle='--')
    plt.show()


def get_data_from_csv(path) -> list:
    tasks = []
    with open(path) as csvDataFile:
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
