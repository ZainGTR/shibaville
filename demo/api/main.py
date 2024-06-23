from fastapi import FastAPI
from ai.txttoimage import GenerateNFT, poll_inference_status
import base64
base_url = ''
model_id = '' # test model
key = ''
secret = ''

auth = base64.b64encode(f'{key}:{secret}'.encode('ascii')).decode('ascii')

headers = {
    'accept': 'application/json',
    'Authorization': f'Basic {auth}'
}

app = FastAPI()

@app.get("/")
async def read_root():
    requestId = await GenerateNFT(1, False, "factory", base_url, model_id, headers)
    res = await poll_inference_status(model_id, requestId, base_url, headers)
    return {"images": res}