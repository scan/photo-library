import { type FunctionComponent, useState } from 'react';

import AddImagesModal from './components/AddImagesModal';

const App: FunctionComponent = () => {
  const [modalOpen, setModalOpen] = useState(false);

  return (
    <>
      <div className="container mx-auto">
        <h1 className="text-5xl font-bold">Welcome to Tauri!</h1>

        <div className="row">
          <button
            className="btn btn-primary"
            type="button"
            onClick={() => setModalOpen(true)}
          >
            Add new Photos
          </button>
        </div>
      </div>
      <AddImagesModal open={modalOpen} onClose={() => setModalOpen(false)} />
    </>
  );
};

export default App;
