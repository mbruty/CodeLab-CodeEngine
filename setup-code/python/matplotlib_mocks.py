import matplotlib.pyplot as plt
import io
import base64

def show():
    bytes = io.BytesIO()
    plt.savefig(bytes, format='jpg')
    bytes.seek(0)
    base64_encoded = base64.b64encode(bytes.read()).decode()
    print("<%media> data:image/jpeg;base64," + base64_encoded)

plt.show = show
