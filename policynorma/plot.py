import csv
from matplotlib import pyplot as plt
import pandas as pd
import numpy as np


def separate_data(file):
    with open(file, "r") as data:
        reader = csv.reader(data)

        with open("goat.csv", "w") as goat, open("tiger.csv", "w") as tiger:
            goat_writer = csv.writer(goat)
            tiger_writer = csv.writer(tiger)

            goat_writer.writerow(
                [
                    "Norma Agent",
                    "Wins",
                    "Loss",
                    "Draw",
                    "Total Games Played",
                    "Predicted Invalid Moves",
                    "Turns",
                ]
            )
            tiger_writer.writerow(
                [
                    "Norma Agent",
                    "Wins",
                    "Loss",
                    "Draw",
                    "Total Games Played",
                    "Predicted Invalid Moves",
                    "Turns",
                ]
            )

            for row in reader:
                if row[0] == "1":
                    goat_writer.writerow(row)

                if row[0] == "-1":
                    tiger_writer.writerow(row)


def plot_data_tiger():
    plt.rcParams["figure.figsize"] = [7.00, 3.50]
    plt.rcParams["figure.autolayout"] = True

    data = pd.read_csv("tiger.csv")
    data.pop("Predicted Invalid Moves")
    data.pop("Turns")

    new_data = data[::5]

    wins = new_data["Wins"]
    loss = new_data["Loss"]
    # draw = new_data["Draw"]
    time = np.arange(len(new_data)) * 2 

    plt.plot(time, wins, label="Wins")
    plt.plot(time, loss, label="Loss")
    # plt.plot(time, draw, label="Draw")

    plt.xlabel("Time Interval")
    plt.ylabel("Number of Games")
    plt.title("Tiger Wins, Loss and Draws Over Time")
    plt.legend()

    plt.savefig("tiger.png", dpi=300, bbox_inches="tight")


def plot_data_goat():
    plt.rcParams["figure.figsize"] = [7.00, 3.50]
    plt.rcParams["figure.autolayout"] = True

    data = pd.read_csv("goat.csv")
    data.pop("Predicted Invalid Moves")
    data.pop("Turns")
    new_data = data.rolling(8).min().iloc[::5]

    wins = new_data["Wins"]
    loss = new_data["Loss"]
    draw = new_data["Draw"]
    time = np.arange(len(new_data)) * 2

    plt.plot(time, wins, label="Wins")
    plt.plot(time, loss, label="Loss")
    plt.plot(time, draw, label="Draw")

    plt.title("Goat Wins, Loss and Draws Over Time")
    plt.legend()

    plt.xlabel("Time Interval")
    plt.ylabel("Number of Games")

    plt.savefig("goat.png", dpi=300, bbox_inches="tight")

def plot_invalid_moves():
    plt.rcParams["figure.figsize"] = [7.00, 3.50]
    plt.rcParams["figure.autolayout"] = True

    data = pd.read_csv("goat.csv")

    invalid_moves = data["Predicted Invalid Moves"].apply(lambda x: [int(i) for i in eval(x)])
    game_length = data["Turns"].apply(lambda x: [int(i) for i in eval(x)])

    plt.title("Goat Wins, Loss and Draws Over Time")
    plt.legend()

    plt.xlabel("Time Interval")
    plt.ylabel("Number of Games")

    # plt.savefig("invalid.png", dpi=300, bbox_inches="tight")

# plot_invalid_moves()
separate_data("gameplayrecord.csv")
plot_data_tiger()
plt.clf()
plot_data_goat()
