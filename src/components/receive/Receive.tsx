import React, { useEffect, useState } from 'react';
import { makeStyles } from '@mui/styles';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import Input from '@mui/material/Input';
import FormControl from '@mui/material/FormControl';
import FormHelperText from '@mui/material/FormHelperText';
import Box from '@mui/material/Box';
import Card from '@mui/material/Card';
import CardActions from '@mui/material/CardActions';
import CardContent from '@mui/material/CardContent';

import { invoke } from '@tauri-apps/api';
import Typography from '@mui/material/Typography';

const Receive = () => {
  const [names, setNames] = useState<string[]>([]);

  useEffect(() => {
    invoke('get_download_files').then((data) => {
      let files = data as string[];
      let file_names = files
        .map((file: string) => {
          return file.split('../downloads/')[1];
        })
        .filter((file: string) => {
          return file !== '.gitkeep';
        });
      setNames(file_names);
    });
  }, []);

  return (
    <Container>
      {names &&
        names.map((name: string) => (
          <Card sx={{ minWidth: 275, marginBottom: 1 }}>
            <CardContent>
              <Typography variant='body2'>File name: {name}</Typography>
            </CardContent>
          </Card>
        ))}

      <div
        style={{
          height: 100,
        }}
      />
    </Container>
  );
};

export default Receive;
