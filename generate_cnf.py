import random

def generate_cnf(num_variables, num_clauses, max_literals_per_clause):
    cnf = []

    # Add clauses
    for _ in range(num_clauses):
        clause = []
        num_literals = random.randint(1, max_literals_per_clause)
        for _ in range(num_literals):
            variable = random.randint(1, num_variables)
            # Randomly negate some literals
            if random.choice([True, False]):
                variable = -variable
            clause.append(variable)
        cnf.append(clause)

    return cnf

def print_dimacs(cnf):
    print("p cnf {} {}".format(num_variables, num_clauses))
    for clause in cnf:
        print(" ".join(map(str, clause)), "0")

if __name__ == "__main__":
    num_variables = 10  # Change this to the desired number of variables
    num_clauses = 4    # Change this to the desired number of clauses
    max_literals_per_clause = 10  # Change this to the desired maximum number of literals per clause

    random.seed(57)  # Set seed for reproducibility
    cnf_instance = generate_cnf(num_variables, num_clauses, max_literals_per_clause)
    print_dimacs(cnf_instance)
