import React, { useEffect, useState } from 'react';
import { makeStyles } from '@mui/styles';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import Input from '@mui/material/Input';
import FormControl from '@mui/material/FormControl';
import FormHelperText from '@mui/material/FormHelperText';

import { invoke } from '@tauri-apps/api';

const SendFile = () => {
  // const [blocks, setBlocks] = useState<any>(null);

  useEffect(() => {
    invoke('get_blocks').then((blocks) => {
      console.log(blocks);
    });
  }, []);

  return <Container>{/* <p>{blocks}</p> */}</Container>;
};

export default SendFile;
