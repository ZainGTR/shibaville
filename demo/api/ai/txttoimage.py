import requests
import time



async def GenerateNFT(tokenId, ville, type, base_url, model_id, headers):
    # Make a POST request to create an inference
    prompt = "isometric"
    if ville:
        prompt += " city hall"
    else:
        prompt += type

    response = requests.post(f'{base_url}/generate/txt2img', json={
            'modelId': model_id,
        'prompt':
            prompt,
        'numInferenceSteps': 30,
        'numSamples': 2,
        'guidance': 7.5,
        'width': 1024,
        'height': 1024,
        'negativePrompt': 'ugly, bad, low quality, blurry',
        }, headers=headers)

    # Check if the request was successful
    if response.status_code == 200:
        data = response.json()
        print(data)
        inference_id = data['inference']['id']
        return inference_id;

    else:
        print(f'Error: {response}')


async def poll_inference_status(model_id, inference_id, base_url, headers):
        status = ''
        while status not in ['succeeded', 'failed']:
            # Fetch the inference details
            inference_response = requests.get(f'{base_url}/models/{model_id}/inferences/{inference_id}', headers=headers)
            inference_data = inference_response.json()
            status = inference_data['inference']['status']
            print(f'Inference status: {status}')

            # Wait for a certain interval before polling again
            time.sleep(5)  # Polling every 5 seconds

        # Handle the final status
        if status == 'succeeded':
            print('Inference succeeded!')
            print(inference_data)  # Print inference data
            images = inference_data['inference']['images']
            return images
        else:
            print('Inference failed!')
            print(inference_data)  # Print inference data
