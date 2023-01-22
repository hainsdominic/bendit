import { useState } from 'react';
import Navigations from './components/Navigations';
import SendFile from './components/send/SendFile';

function App() {
    const [value, setValue] = useState(0);

    const ShowPages = () => {
        if (value === 0) {
            return <SendFile />;
        } else if (value === 1) {
            return <h1> receive</h1>;
        } else if (value === 2) {
            return <h1> explorer</h1>;
        }
        return <></>;
    };

    return (
        <div>
            <ShowPages />
            <div>
                <Navigations value={value} setValue={setValue} />
            </div>
        </div>
    );
}

export default App;
