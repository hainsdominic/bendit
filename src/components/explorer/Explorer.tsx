import React, { useEffect, useState } from "react";
import { makeStyles } from "@mui/styles";
import Container from "@mui/material/Container";
import Grid from "@mui/material/Grid";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import Input from "@mui/material/Input";
import FormControl from "@mui/material/FormControl";
import FormHelperText from "@mui/material/FormHelperText";
import Box from "@mui/material/Box";
import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";

import { invoke } from "@tauri-apps/api";
import Typography from "@mui/material/Typography";

const SendFile = () => {
  const [blocks, setBlocks] = useState<any>(null);

  useEffect(() => {
    fetch("src/assets/blockchain.json")
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        setBlocks(data);
      });
  }, []);

  return (
    <Container>
      {blocks &&
        blocks.map((block: any) => (
          <Card sx={{ minWidth: 275, marginBottom: 1 }}>
            <CardContent>
              <Typography variant="h5">Index: {block.index}</Typography>
              <Typography variant="body2">
                Timestamp: {block.timestamp}
              </Typography>
              <Typography variant="body2">Sender: {block.sender_pub_key}</Typography>
              <Typography variant="body2">
                Receiver: {block.receiver_pub_key}
              </Typography>
              <Typography variant="body2">
                File hash: {block.file_hash}
              </Typography>
              <Typography variant="body2">
                File hash: {block.sender_signature}
              </Typography>
              <Typography variant="body2">
                Previous hash: {block.prev_block_hash}
              </Typography>
              <Typography variant="body2">
                Block hash: {block.block_hash}
              </Typography>
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

export default SendFile;
