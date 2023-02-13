import pandas as pd
import joblib

def make_prediction(input_file: str, model_file: str, output_file: str):
    # Load the preprocessed data into a pandas DataFrame
    data = pd.read_csv(input_file)

    # Split the data into features
    X = data

    # Load the trained model from a file
    model = joblib.load(model_file)

    # Make predictions using the trained model
    y_pred = model.predict(X)

    # Save the predictions to a file
    pd.DataFrame({'prediction': y_pred}).to_csv(output_file, index=False)

if __name__ == '__main__':
    # Example usage of the make_prediction function
    input_file = 'preprocessed_data.csv'
    model_file = 'trained_model.joblib'
    output_file = 'predictions.csv'
    make_prediction(input_file, model_file, output_file)
