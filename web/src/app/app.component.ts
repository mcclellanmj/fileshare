import { Component } from '@angular/core';
import { FileSharingService } from './file-sharing.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
  providers: [FileSharingService]
})

export class AppComponent { }
