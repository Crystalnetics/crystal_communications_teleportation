import pandas as pd
from sklearn.ensemble import RandomForestRegressor

def train_model(input_file: str, output_file: str):
    # Load the preprocessed data into a pandas DataFrame
    data = pd.read_csv(input_file)

    # Split the data into features and target
    X = data.drop('target', axis=1)
    y = data['target']

    # Train a Random Forest Regressor
    model = RandomForestRegressor()
    model.fit(X, y)

    # Save the trained model to a file
    import joblib
    joblib.dump(model, output_file)

if __name__ == '__main__':
    # Example usage of the train_model function
    input_file = 'preprocessed_data.csv'
    output_file = 'trained_model.joblib'
    train_model(input_file, output_file)
