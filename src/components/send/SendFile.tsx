import React, { useState } from 'react';
import { makeStyles } from '@mui/styles';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import Input from '@mui/material/Input';
import FormControl from '@mui/material/FormControl';
import FormHelperText from '@mui/material/FormHelperText';

import { invoke } from '@tauri-apps/api';

const useStyles = makeStyles((theme) => ({
    root: {
        '& .MuiTextField-root': {
            margin: 1,
        },
    },
    container: {
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        transform: 'translateX(50%) scale(1.4)',
        padding: 5,
        paddingBottom: 10,
        backgroundColor: 'white',
        borderRadius: 10,
        width: '60%',
        height: '50%',
    },
}));

const SendFile = () => {
    const classes = useStyles();
    const [file, setFile] = useState(null);
    const [publicKey, setPublicKey] = useState('');
    const [error, setError] = useState(false);
    const [helperText, setHelperText] = useState('');

    const handleFileChange = (event: any) => {
        setFile(event.target?.files[0]);
    };

    const handlePublicKeyChange = (event: any) => {
        setPublicKey(event.target.value);
    };

    const handleSubmit = (event: any) => {
        event.preventDefault();
        if (!file) {
            setError(true);
            setHelperText('Please select a file to send');
        } else if (!publicKey) {
            setError(true);
            setHelperText('Please enter a public key');
        } else {
            let ip: string;
            invoke('get_recipient_ip', { publicKey: publicKey }).then(
                (res: any) => {
                    ip = res.replace(/(\r\n|\n|\r)/gm, '') + ':8080';
                    const reader = new FileReader();
                    reader.onloadend = () => {
                        const arrayBuffer = reader.result;
                        invoke('send_file', {
                            ip: ip,
                            fileBuffer: arrayBuffer?.toString(),
                            fileName: (file as File).name,
                        });
                    };
                    reader.readAsArrayBuffer(file);
                }
            );
        }
    };

    return (
        <Container className={classes.container}>
            <form className={classes.root} onSubmit={handleSubmit}>
                <Grid container direction='column'>
                    <FormControl sx={{ margin: 2 }}>
                        <Input
                            type='file'
                            id='file-input'
                            onChange={handleFileChange}
                        />
                        <FormHelperText>
                            Select the file you want to send
                        </FormHelperText>
                    </FormControl>
                    <TextField
                        id='public-key'
                        label='Public Key'
                        variant='outlined'
                        value={publicKey}
                        onChange={handlePublicKeyChange}
                        error={error}
                        helperText={helperText}
                        sx={{ alignSelf: 'center' }}
                    />
                    <Button
                        type='submit'
                        variant='contained'
                        color='primary'
                        sx={{ margin: 2 }}
                    >
                        Send
                    </Button>
                </Grid>
            </form>
        </Container>
    );
};

export default SendFile;
