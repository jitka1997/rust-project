import json
from typing import List
import matplotlib.pyplot as plt


def load_results(filename) -> List[int]:
    """Opens and parse a JSON file"""
    with open(filename, "r") as fichier:
        results = json.load(fichier)
    return results


def compute_proportions(
    results: List[int],
) -> tuple[List[float], List[float], List[float]]:
    """Takes a list of outcomes and computes the proportion of victories, ties and defeats."""
    nb_victory = 0
    nb_tie = 0
    nb_defeat = 0

    proportion_victory = []
    proportion_tie = []
    proportion_defeat = []
    for result in results:
        if result == -1:
            nb_defeat += 1
        elif result == 0:
            nb_tie += 1
        elif result == 1:
            nb_victory += 1
        else:
            print("error in parsing results")
            exit()
        total = nb_victory + nb_tie + nb_defeat
        proportion_victory.append(nb_victory / total)
        proportion_tie.append(nb_tie / total)
        proportion_defeat.append(nb_defeat / total)
    return proportion_victory, proportion_tie, proportion_defeat


def main():
    results = load_results("results.json")
    proportion_victory, proportion_tie, proportion_defeat = compute_proportions(results)

    plt.plot(range(len(proportion_victory)), proportion_victory, label="victory")
    plt.plot(range(len(proportion_tie)), proportion_tie, label="tie")
    plt.plot(range(len(proportion_defeat)), proportion_defeat, label="defeat")
    plt.legend(loc="best")
    plt.xlabel("number of iteration")
    plt.ylabel("proportion of victory")
    plt.show()


if __name__ == "__main__":
    main()
