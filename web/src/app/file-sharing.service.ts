import { Injectable } from '@angular/core';
import { Http } from '@angular/http';

import 'rxjs/add/operator/toPromise';

@Injectable()
export class FileSharingService {
  private readonly fileListUrl: string = '/view?folder_path=???';

  constructor(private http: Http) { }

  getFiles(directory: string): Promise<File[]> {
    return this.http.get(this.fileListUrl)
               .toPromise()
               .then(response => response.json().data as File[])
               .catch(this.handleError);
  }

  private handleError(error: Any): Promise<any> {
    console.error('An error occured', error);
    return Promise.reject(error.message || error);
  }
}
