import pandas as pd
from sklearn.metrics import mean_squared_error
import joblib

def evaluate_model(input_file: str, model_file: str):
    # Load the preprocessed data into a pandas DataFrame
    data = pd.read_csv(input_file)

    # Split the data into features and target
    X = data.drop('target', axis=1)
    y = data['target']

    # Load the trained model from a file
    model = joblib.load(model_file)

    # Make predictions using the trained model
    y_pred = model.predict(X)

    # Calculate the mean squared error
    mse = mean_squared_error(y, y_pred)
    print(f"Mean Squared Error: {mse}")

if __name__ == '__main__':
    # Example usage of the evaluate_model function
    input_file = 'preprocessed_data.csv'
    model_file = 'trained_model.joblib'
    evaluate_model(input_file, model_file)
